use dialoguer::{theme::ColorfulTheme, Select};
use serialport;
use std::io::{self, Read, Write};
use std::time::Duration;

fn main() {
    println!("=== UART CLI Listener ===");

    // Автоматический поиск доступных портов
    let ports = match serialport::available_ports() {
        Ok(p) if !p.is_empty() => p,
        Ok(_) => {
            eprintln!("Доступные COM-порты не найдены. Проверьте подключение устройства.");
            return;
        }
        Err(e) => {
            eprintln!("Не удалось получить список портов: {}", e);
            return;
        }
    };

    // Создаем текстовый список портов для меню
    let port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();

    // Интерактивный выбор порта
    let port_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Выберите устройство (порт) для подключения")
        .default(0)
        .items(&port_names[..])
        .interact()
        .unwrap();

    let selected_port = &port_names[port_selection];

    // Интерактивный выбор скорости (baud_rate)
    let speeds = vec!["9600", "19200", "38400", "57600", "115200"];
    let speed_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Выберите скорость (Baud Rate)")
        .default(4) // По умолчанию 115200
        .items(&speeds[..])
        .interact()
        .unwrap();

    let selected_baud: u32 = speeds[speed_selection].parse().unwrap();

    println!("\n▶Подключение к {} на скорости {}...", selected_port, selected_baud);

    // Открытие выбранного порта
    let port = serialport::new(selected_port, selected_baud)
        .timeout(Duration::from_secs(1))
        .open();

    let mut port = match port {
        Ok(p) => {
            println!("Успешно подключено! Нажмите Ctrl+C для выхода.\n---");
            p
        }
        Err(e) => {
            eprintln!("Ошибка открытия порта: {}", e);
            return;
        }
    };

    // Бесконечный цикл считывания информации
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Выводим полученные данные на экран в UTF-8
                let text = String::from_utf8_lossy(&buffer[..bytes_read]);
                print!("{}", text);
                
                // Сбрасываем буфер вывода для мгновенного отображения символов
                let _ = io::stdout().flush();
            }
            Ok(_) => {} // Нулевой буфер при таймауте, просто продолжаем
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {} // Игнорируем таймауты
            Err(e) => {
                eprintln!("\nСоединение разорвано или произошла ошибка: {}", e);
                break;
            }
        }
    }
}