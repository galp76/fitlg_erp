use std::net::{ TcpListener, TcpStream };
use std::io::{ prelude::*, BufReader };
use std::fs;
use std::{ io::Write };
use chrono::{Datelike, Utc};

fn clean_file(file_name: String) {
    std::fs::OpenOptions::new()
	.create(true)
	.write(true)
	.truncate(true)
	.open(file_name)
	.unwrap();
}

fn appendToFile(fileName: String, data: String) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(fileName)
        .unwrap();
    write!(file, "{}\n", &data).expect("Unable to write data");
}

fn createFile(data: String) {
    fs::File::create(format!("dates/{}.txt", data)).expect("Unable to create file");
}

fn getDate() -> String {
    let now = Utc::now();
    let result = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());

    result
}

fn split76(text: &str) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    let mut tmp: String = String::new();
    for ch in text.chars() {
        match ch {
            ' ' => {
                answer.push(String::from(&tmp));
                tmp.clear();
            },
            _ => tmp.push(ch)
                
        }
    }
    answer.push(tmp);

    answer
}

pub fn set_alarm(seconds: usize) {
    let stop_flag = std::sync::atomic::AtomicBool::new(false);
    let counter = std::sync::atomic::AtomicUsize::new(0);
    let limit = std::sync::atomic::AtomicUsize::new(seconds);
    std::thread::scope(|s| {
        s.spawn(|| {
            while !stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                spin_sleep::sleep(std::time::Duration::new(1, 0));
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if counter.load(std::sync::atomic::Ordering::Relaxed) >= limit.load(std::sync::atomic::Ordering::Relaxed) { break; }     
            }
	    // Para que no suene la alarma cuando se usa "stop":
	    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
		std::process::exit(0);
	    }
            let mut child = std::process::Command::new("mplayer")
                .arg("bell.mp3")
                .spawn()
                .expect("Falla al ejecutar mplayer");   
            child.wait().expect("Failed to wait child process mplayer");
            println!("Recordatorio para flexiones");
            std::process::exit(0);
        });

        println!("Iniciando {seconds} segundos para recordatorio.");
        println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
        println!("Indique un comando:");
        for line in std::io::stdin().lines() {
	    let string = line.unwrap();
	    if &string == "" {
		println!("No se recibio ningun comando.");
		println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
		println!("Indique un comando:");
		continue;
	    }
	    let commands: Vec<&str> = string.split(' ').collect();
	    match commands[0] {
		"stop" => break,
		"show" => println!("Counter: {}", counter.load(std::sync::atomic::Ordering::Relaxed)),
		"f" if commands.len() == 3 => {
		    data::newData(commands[0], commands[1]);
		    counter.store(0, std::sync::atomic::Ordering::Relaxed);
		    limit.store(commands[2].parse::<usize>().unwrap(), std::sync::atomic::Ordering::Relaxed);
		    println!("Reiniciando contador para recordatorio en {} segundos", limit.load(std::sync::atomic::Ordering::Relaxed));
		},
		cmd => println!("{cmd:?} no es un comando valido.\n"),
	    }
            println!("Comandos disponibles: stop / show / f [cantidad] [segundos para recordatorio]");
            println!("Indique un comando:");
        }

        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);

    });
}

pub mod data {

use std::{ fs, process, error::Error };

    fn changeChar(file: &str, actual: char, new: char) -> Result<String, Box<dyn Error>> {
        let mut answer: String = format!("{};", &file[..file.len()-4]);
        let entries = fs::read_to_string(file)?;
        for ch in entries.chars() {
            if ch != actual {
                answer.push(ch);
            } else {
                answer.push(new);
            }
        }

        Ok(answer)
    }

    pub fn fileToString(fileName: &str) {
        let data = changeChar(fileName, '\n', ';');
        match data {
            Ok(x) => super::appendToFile("resumen.txt".to_string(), x),
            _ => println!("Error procesando el archivo {}", fileName),
        }
    }

    fn dayInfo(letter: &str, date: String) -> (&str, u8) { 
        let item = match letter {
            "f" => "flexiones",
            "c" => "caminata",
            "r" => "remo",
            _ => "peso",
        };

        let mut counter: u8 = 0;
        let entries = fs::read_to_string(format!("dates/{}.txt", date)).unwrap_or_else(|error| {
            eprintln!("Se encontró un error al tratar de leer el archivo {}: {}", date, error);
            process::exit(1);
        });

        for line in entries.lines() {
            let elements = super::split76(line);
            if elements[0] == letter {
                counter += elements[1].parse::<u8>().unwrap();
            }
        }

        (item, counter)
    }
    
    pub fn get_graph_array(exercise: &str) -> String {
        use chrono::{ Local, Datelike, DateTime, Duration, Utc };
        let day_of_week: usize = Local::now().weekday() as usize;
        /*if day_of_week == 0 {
            return "[0, 0, 0, 0, 0, 0, 0]".to_string();
        }*/
        let mut result = vec![0; 7];
        let dates_vector: Vec<String> = std::fs::read_to_string(format!("dates.txt"))
            .unwrap()
            .split("\n")
            .map(|it| it.to_string())
            .filter(|it| it.len() != 0)
            .collect();
        println!("{:?}", dates_vector);
        // len y counter: vamos incrementando counter para iterar hacia atras en el vector result, usando day_of_week - counter
        let mut counter: usize = 0;
        // utc_now: para hacer operaciones con las fechas usando chrono
        let mut utc_now = Utc::now();
        loop {
            // start_date: para convertir las fechas de utc_now en un String YYYY-MM-DD
            let mut start_date: String = format!("{}-{:02}-{:02}", utc_now.year(), utc_now.month(), utc_now.day());
            // IMPRIMIENDO start_date PARA DEBUGGING:
            println!("{}", start_date);

            let mut created = false;
            let dates = fs::read_to_string("dates.txt").unwrap_or_else(|err| {
                eprintln!("No fue posible leer el archivo dates.txt: {err}");
                process::exit(1);
            });

            for line in dates.lines() {
                if line == format!("{}", start_date) { 
                    created = true;
                    break;
                }
            }

            if !created {
                result[day_of_week - counter] = 0;
                if counter == day_of_week { 
                    break;
                }
                counter += 1;
                utc_now = utc_now.checked_sub_signed(Duration::days(1))
                    .unwrap();
                continue;
            }

            let (_, repetitions) = dayInfo(exercise, start_date);
            result[day_of_week - counter] = repetitions;
            if counter == day_of_week { 
                break;
            }
            utc_now = utc_now.checked_sub_signed(Duration::days(1))
                .unwrap();
            counter += 1;
        }

        let mut string = format!("[{}", result[0]);
        for i in 1..result.len() {
            if i != result.len() - 1 {
                string = format!("{}, {}", string, result[i]);
            } else {
                string = format!("{}, {}]", string, result[i]);
            }
        }

        string
    }
    
    pub fn newData(exercise: &str, quantity: &str) {
        let number = quantity.parse::<u8>().unwrap_or_else(|error| {
            eprintln!("El segundo argumento no es válido: {}", error);
            process::exit(1);
        });
        let cldata = format!("{} {}", exercise, number);
        let dateStr = super::getDate();
        let mut created = false;
        let dates = fs::read_to_string("dates.txt").unwrap_or_else(|err| {
            eprintln!("No fue posible leer el archivo dates.txt: {err}");
            process::exit(1);
        });

        for line in dates.lines() {
            if line == format!("{}", dateStr) { 
                created = true;
                break;
            }
        }

        if !created {
            super::createFile(dateStr.clone());
            super::appendToFile("dates.txt".to_string(), dateStr.clone());
        }

        super::appendToFile(format!("dates/{}.txt", dateStr), cldata);
        let (item, counter) = dayInfo(exercise, dateStr);
        println!("Registro sobre {} guardado.", item);
        if item != "peso" {
            println!("Acumulado del día: {}", counter);
            if item == "flexiones" {
                use chrono::{ Utc, Timelike };
                let now = Utc::now();
                println!("Hora: {:02}:{:02}", now.hour()-4, now.minute());
            }
        }

        // Actualizamos hello.xhtml con los nuevos datos de ejercicios
        super::clean_file("tmp.xhtml".to_string());
        let first_half = std::fs::read_to_string("hello_first_half.html").unwrap();
        super::appendToFile("tmp.xhtml".to_string(), first_half);

        let data_array = get_graph_array("r");
        super::appendToFile("tmp.xhtml".to_string(), format!("data: {},", data_array));
        let second_half = std::fs::read_to_string("hello_second_half.html").unwrap();
        super::appendToFile("tmp.xhtml".to_string(), second_half);
        std::fs::rename("tmp.xhtml", "hello.xhtml");
    }
}
