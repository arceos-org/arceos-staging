use crate::io::AxPollState;
use axerrno::AxResult;
use axnet::{TcpSocket, UdpSocket};
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use axsync::Mutex;

pub struct AxTcpSocketHandle(TcpSocket);
pub struct AxUdpSocketHandle(Mutex<UdpSocket>);

pub enum AxSocketHandle {
    Tcp(AxTcpSocketHandle),
    Udp(AxUdpSocketHandle),
}

const fn into_ax_ipaddr(ip: IpAddr) -> axnet::IpAddr {
    match ip {
        IpAddr::V4(ip) => axnet::IpAddr::Ipv4(axnet::Ipv4Addr(ip.octets())),
        _ => panic!("IPv6 not supported"),
    }
}

const fn into_core_ipaddr(ip: axnet::IpAddr) -> IpAddr {
    match ip {
        axnet::IpAddr::Ipv4(ip) => IpAddr::V4(unsafe { core::mem::transmute(ip.0) }),
    }
}

const fn into_ax_sockaddr(addr: SocketAddr) -> axnet::SocketAddr {
    axnet::SocketAddr::new(into_ax_ipaddr(addr.ip()), addr.port())
}

const fn into_core_sockaddr(addr: axnet::SocketAddr) -> SocketAddr {
    SocketAddr::new(into_core_ipaddr(addr.addr), addr.port)
}

pub fn ax_tcp_socket() -> AxTcpSocketHandle {
    AxTcpSocketHandle(TcpSocket::new())
}

pub fn ax_tcp_socket_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr> {
    socket.0.local_addr().map(into_core_sockaddr)
}

pub fn ax_tcp_peer_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr> {
    socket.0.peer_addr().map(into_core_sockaddr)
}

pub fn ax_tcp_set_nonblocking(socket: &AxTcpSocketHandle, nonblocking: bool) -> AxResult {
    socket.0.set_nonblocking(nonblocking);
    Ok(())
}

pub fn ax_tcp_connect(socket: &AxTcpSocketHandle, addr: SocketAddr) -> AxResult {
    socket.0.connect(into_ax_sockaddr(addr))
}

pub fn ax_tcp_bind(socket: &AxTcpSocketHandle, addr: SocketAddr) -> AxResult {
    socket.0.bind(into_ax_sockaddr(addr))
}

pub fn ax_tcp_listen(socket: &AxTcpSocketHandle, _backlog: usize) -> AxResult {
    socket.0.listen()
}

pub fn ax_tcp_accept(socket: &AxTcpSocketHandle) -> AxResult<(AxTcpSocketHandle, SocketAddr)> {
    let new_sock = socket.0.accept()?;
    let addr = new_sock.peer_addr().map(into_core_sockaddr)?;
    Ok((AxTcpSocketHandle(new_sock), addr))
}

pub fn ax_tcp_send(socket: &AxTcpSocketHandle, buf: &[u8]) -> AxResult<usize> {
    socket.0.send(buf)
}

pub fn ax_tcp_recv(socket: &AxTcpSocketHandle, buf: &mut [u8]) -> AxResult<usize> {
    socket.0.recv(buf)
}

pub fn ax_tcp_poll(socket: &AxTcpSocketHandle) -> AxResult<AxPollState> {
    socket.0.poll()
}

pub fn ax_tcp_shutdown(socket: &AxTcpSocketHandle) -> AxResult {
    socket.0.shutdown()
}

pub fn ax_get_addr_info(
    domain_name: &str,
    port: Option<u16>,
) -> AxResult<alloc::vec::Vec<SocketAddr>> {
    Ok(axnet::resolve_socket_addr(domain_name)?
        .into_iter()
        .map(|ip| SocketAddr::new(into_core_ipaddr(ip), port.unwrap_or(0)))
        .collect())
}

pub fn ax_poll_interfaces() -> AxResult {
    axnet::poll_interfaces();
    Ok(())
}

pub fn ax_udp_bind(addr: SocketAddr) -> AxResult<AxUdpSocketHandle> {
    let socket = AxUdpSocketHandle(Mutex::new(UdpSocket::new()));
    let _= socket.0.lock().bind(into_ax_sockaddr(addr));
    Ok(socket)
}

pub fn ax_udp_connect(socket: &AxUdpSocketHandle, addr: SocketAddr) -> AxResult {
    socket.0.lock().connect(into_ax_sockaddr(addr))
}

pub fn ax_udp_poll(socket: &AxUdpSocketHandle) -> AxResult<AxPollState> {
    socket.0.lock().poll()
}

pub fn ax_udp_send(socket: &AxUdpSocketHandle, buf: &[u8]) -> AxResult<usize> {
    socket.0.lock().send(buf)
}

pub fn ax_udp_send_to(socket: &AxUdpSocketHandle, buf: &[u8], addr: SocketAddr) -> AxResult<usize> {
    socket.0.lock().send_to(buf, into_ax_sockaddr(addr))
}

pub fn ax_udp_recv_from(socket: &AxUdpSocketHandle, buf: &mut [u8]) -> AxResult<(usize, SocketAddr)> {
    let (size, addr) = socket.0.lock().recv_from(buf)?;
    Ok((size, into_core_sockaddr(addr)))
}

pub fn ax_udp_socket_addr(socket: &AxUdpSocketHandle) -> AxResult<SocketAddr> {
    socket.0.lock().local_addr().map(into_core_sockaddr)
}