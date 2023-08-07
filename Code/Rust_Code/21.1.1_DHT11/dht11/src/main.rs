mod dht;
use anyhow::Result;
use dht::DHT;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Program is starting ...");
    let mut dht = DHT::new(17)?;

    let mut counts: usize = 0;
    loop {
        counts += 1;
        println!("Measurement counts : {counts} ");

        if let Ok(_) = dht.read() {
            println!("DHT11,OK! ");

            let humidity = dht.get_humidity();
            let temperature = dht.get_temperature();

            println!("Humidity is {humidity:.2} %, \t Temperature is {temperature:.2} ℃ \n");
        } else {
            println!("DHT11,Error! ");
        }
        thread::sleep(Duration::from_millis(2000));
    }
}
