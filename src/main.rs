// Cargo.toml:
// termion = "1.5"
// chrono  = "0.4"

use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use std::fs::File;

use termion::{clear, cursor, color, style, input::TermRead, event::Key};
use termion::terminal_size;
use chrono::Local;

// Converte Ipv4Addr <-> u32 para iterar ranges
fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    let oct = ip.octets();
    ((oct[0] as u32) << 24)
        | ((oct[1] as u32) << 16)
        | ((oct[2] as u32) << 8)
        |  (oct[3] as u32)
}

// Converte u32 de volta para Ipv4Addr
fn u32_to_ipv4(v: u32) -> Ipv4Addr {
    Ipv4Addr::new(
        ((v >> 24) & 0xFF) as u8,
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8)  & 0xFF) as u8,
         (v & 0xFF) as u8
    )
}

// Limpa a tela
fn clear_screen() {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
}

// Desenha contorno azul
fn draw_border() -> io::Result<()> {
    let (cols, rows) = terminal_size()?;
    let width = cols as usize;
    let height = rows as usize;
    print!("{}{}", color::Bg(color::Blue), color::Fg(color::Yellow));
    print!("{}┌{}┐", cursor::Goto(1, 1), "─".repeat(width.saturating_sub(2)));
    for row in 2..height {
        print!("{}│{}│", cursor::Goto(1, row as u16), " ".repeat(width.saturating_sub(2)));
    }
    print!("{}└{}┘", cursor::Goto(1, height as u16), "─".repeat(width.saturating_sub(2)));
    Ok(())
}

// Desenha menu principal
fn draw_menu() -> io::Result<()> {
    clear_screen();
    draw_border()?;
    let (cols, _) = terminal_size()?;
    let title = "Brazucascan";
    let subtitle = "by Tony Garcia";
    let tx = cols.saturating_sub(title.len() as u16) / 2;
    let sx = cols.saturating_sub(subtitle.len() as u16) / 2;
    print!("{}{}{}{}{}",
        cursor::Goto(tx.max(1), 2), color::Fg(color::LightYellow), title,
        cursor::Goto(sx.max(1), 3), subtitle
    );
    print!("{}", color::Fg(color::Yellow));
    let items = [
        "1) Scanner por endereço (faixa de portas)",
        "2) Scanner por range de endereço",
        "3) Sair",
    ];
    for (i, item) in items.iter().enumerate() {
        print!("{}{}", cursor::Goto(3, (5 + i) as u16), item);
    }
    let pr = 5 + items.len() as u16 + 1;
    print!("{}{}{}Escolha uma opção: ",
        cursor::Goto(3, pr), color::Bg(color::Blue), color::Fg(color::Yellow)
    );
    io::stdout().flush()?;
    Ok(())
}

// Lê linha de stdin
fn read_line() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

// Opção 1: IP único, faixa de portas
fn scan_by_address() -> io::Result<()> {
    clear_screen();
    draw_border()?;
    let (cols, _) = terminal_size()?;
    let title = "Brazucascan";
    let subtitle = "Scanner por endereço";
    let tx = cols.saturating_sub(title.len() as u16) / 2;
    let sx = cols.saturating_sub(subtitle.len() as u16) / 2;
    print!("{}{}{}{}{}",
        cursor::Goto(tx.max(1), 2), color::Fg(color::LightYellow), title,
        cursor::Goto(sx.max(1), 3), subtitle
    );
    print!("{}{}", color::Bg(color::Blue), color::Fg(color::Yellow));
    print!("{}Digite o IPv4: ", cursor::Goto(3, 5)); io::stdout().flush()?;
    let ip = IpAddr::from_str(&read_line()?)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "IP inválido"))?;
    print!("{}Porta inicial: ", cursor::Goto(3, 7)); io::stdout().flush()?;
    let start: u16 = read_line()?.parse().unwrap_or(1);
    print!("{}Porta final:   ", cursor::Goto(3, 8)); io::stdout().flush()?;
    let end: u16 = read_line()?.parse().unwrap_or(start);
    let total = (end - start + 1) as f64;
    let bw = 50;
    let br = 10;
    let mut or = br + 2;
    for (i, port) in (start..=end).enumerate() {
        let pct = ((i + 1) as f64 / total * 100.0).round() as u8;
        let fill = ((pct as f64 / 100.0) * bw as f64).round() as usize;
        print!("{}[{}{}] {:>3}%", cursor::Goto(3, br),
            "#".repeat(fill), " ".repeat(bw - fill), pct
        );
        io::stdout().flush()?;
        if TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_millis(100)).is_ok() {
            print!("{}• Porta {} aberta", cursor::Goto(3, or), port);
            io::stdout().flush()?;
            or += 1;
        }
    }
    // gerar log
    print!("{}Gerar .log? (y/n): ", cursor::Goto(3, or + 1)); io::stdout().flush()?;
    if read_line()?.eq_ignore_ascii_case("y") {
        let fname = Local::now().format("%Y%m%d_%H%M%S.log").to_string();
        let mut f = File::create(&fname)?;
        writeln!(f, "IP escaneado: {}", ip)?;
        for port in start..=end {
            if TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_millis(100)).is_ok() {
                writeln!(f, "Porta {} aberta", port)?;
            }
        }
        print!("{}Arquivo de log gerado, pressione R para retornar", cursor::Goto(3, or + 2));
        io::stdout().flush()?;
        for key in io::stdin().keys() {
            if let Ok(Key::Char(c)) = key {
                if c == 'r' || c == 'R' {
                    break;
                }
            }
        }
        clear_screen();
        return Ok(());
    }
    print!("{}Pressione Enter para voltar ao menu.", cursor::Goto(3, or + 4));
    let _ = read_line();
    clear_screen();
    Ok(())
}

// Opção 2: range de IP e portas
fn scan_by_range_address() -> io::Result<()> {
    clear_screen();
    draw_border()?;
    let (cols, _) = terminal_size()?;
    let title = "Brazucascan";
    let subtitle = "Scanner por range de endereço";
    let tx = cols.saturating_sub(title.len() as u16) / 2;
    let sx = cols.saturating_sub(subtitle.len() as u16) / 2;
    print!("{}{}{}{}{}",
        cursor::Goto(tx.max(1), 2), color::Fg(color::LightYellow), title,
        cursor::Goto(sx.max(1), 3), subtitle
    );
    print!("{}{}", color::Bg(color::Blue), color::Fg(color::Yellow));
    print!("{}Inicio IPv4: ", cursor::Goto(3, 5)); io::stdout().flush()?;
    let start_ip = Ipv4Addr::from_str(&read_line()?)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "IP inválido"))?;
    print!("{}Fim    IPv4: ", cursor::Goto(3, 6)); io::stdout().flush()?;
    let end_ip = Ipv4Addr::from_str(&read_line()?)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "IP inválido"))?;
    print!("{}Porta inicial: ", cursor::Goto(3, 8)); io::stdout().flush()?;
    let sp: u16 = read_line()?.parse().unwrap_or(1);
    print!("{}Porta final:   ", cursor::Goto(3, 9)); io::stdout().flush()?;
    let ep: u16 = read_line()?.parse().unwrap_or(sp);
    let ip0 = ipv4_to_u32(start_ip);
    let ip1 = ipv4_to_u32(end_ip);
    let total = ((ip1 - ip0 + 1) as f64) * ((ep - sp + 1) as f64);
    let bw = 50;
    let br = 11;
    let mut or = br + 2;
    let mut idx = 0f64;
    for ip_u in ip0..=ip1 {
        let ip = u32_to_ipv4(ip_u);
        for port in sp..=ep {
            idx += 1.0;
            let pct = ((idx / total * 100.0).round()) as u8;
            let fill = ((pct as f64 / 100.0) * bw as f64).round() as usize;
            print!("{}[{}{}] {:>3}%", cursor::Goto(3, br),
                "#".repeat(fill), " ".repeat(bw - fill), pct
            );
            io::stdout().flush()?;
            if TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(ip), port), Duration::from_millis(100)).is_ok() {
                print!("{}• {}:{} aberta", cursor::Goto(3, or), ip, port);
                io::stdout().flush()?;
                or += 1;
            }
        }
    }
    // gerar log
    print!("{}Gerar .log? (y/n): ", cursor::Goto(3, or + 1)); io::stdout().flush()?;
    if read_line()?.eq_ignore_ascii_case("y") {
        let fname = Local::now().format("%Y%m%d_%H%M%S.log").to_string();
        let mut f = File::create(&fname)?;
        writeln!(f, "IP range escaneado: {} - {}", start_ip, end_ip)?;
        for ip_u in ip0..=ip1 {
            let ip = u32_to_ipv4(ip_u);
            for port in sp..=ep {
                if TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(ip), port), Duration::from_millis(100)).is_ok() {
                    writeln!(f, "{}:{} aberta", ip, port)?;
                }
            }
        }
        print!("{}Arquivo de log gerado, pressione R para retornar", cursor::Goto(3, or + 2));
        io::stdout().flush()?;
        for key in io::stdin().keys() {
            if let Ok(Key::Char(c)) = key {
                if c == 'r' || c == 'R' {
                    break;
                }
            }
        }
        clear_screen();
        return Ok(());
    }
    print!("{}Pressione Enter para voltar ao menu.", cursor::Goto(3, or + 4));
    let _ = read_line();
    clear_screen();
    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        draw_menu()?;
        match read_line()?.as_str() {
            "1" => scan_by_address()?,
            "2" => scan_by_range_address()?,
            "3" => {
                clear_screen();
                // reset to default colors
                print!("{}{}", style::Reset, clear::All);
                io::stdout().flush()?;
                break;
            },
            _   => { println!("\nOpção inválida"); let _ = read_line(); clear_screen(); }
        }
    }
    Ok(())
}
