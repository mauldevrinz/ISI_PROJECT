use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Konfigurasi koneksi serial
    let tty_path = "/dev/ttyUSB0"; 
    let slave_address = 0x01;      
    let baud_rate = 9600;          

    let builder = tokio_serial::new(tty_path, baud_rate)
        .timeout(Duration::from_millis(1000))
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One);

    let port = SerialStream::open(&builder)?;
    let mut ctx = rtu::connect(port).await?;
    ctx.set_slave(Slave(slave_address));

    loop {
        // Membaca temperatur (input register 0x0001)
        let temp_reg = ctx.read_input_registers(0x0001, 1).await?;
        let raw_temp = temp_reg[0];
        let temperature = convert_to_float(raw_temp, 10.0);

        // Membaca kelembaban (input register 0x0002)
        let hum_reg = ctx.read_input_registers(0x0002, 1).await?;
        let raw_hum = hum_reg[0];
        let humidity = convert_to_float(raw_hum, 10.0);

        // Menampilkan hasil pembacaan
        println!("Temperature: {:.1}°C", temperature);
        println!("Humidity: {:.1}%", humidity);

        // Menunggu 2 detik sebelum membaca lagi
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

// Fungsi untuk konversi nilai raw ke float dengan handling nilai negatif
fn convert_to_float(raw_value: u16, divisor: f32) -> f32 {
    if raw_value > 32767 {
        // Handle negative values (two's complement)
        (raw_value as i16) as f32 / divisor
    } else {
        raw_value as f32 / divisor
    }
}
