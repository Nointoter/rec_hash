use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;

// Определяем тип для кэша
type Cache = HashMap<u64, u64>;

// Рекурсивная функция с кэшированием
fn collatz(n: u64, cache: &mut Cache) -> u64 {
    if n == 1 {
        return 0;
    }
    
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    let next = if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }; 

    let result = 1 + collatz(next, cache);

    cache.insert(n, result);

    result
}

// Функция для чтения из файла и заполнения кэша
fn load_cache_from_file(filename: &str, cache: &mut Cache) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(", ").collect();
        
        if parts.len() >= 2 {
            // Парсим значение из строки "n = result"
            let n_part = parts[0].split('=').nth(1).unwrap().trim();
            let result_part = parts[1].split('=').nth(1).unwrap().trim();
            
            let n: u64 = n_part.parse().unwrap();
            let result: u64 = result_part.parse().unwrap();
            
            cache.insert(n, result);
        }
    }
    
    Ok(())
}

// Функция для сохранения кэша в файл
fn save_cache_to_file(filename: &str, cache: &Cache) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for (key, value) in cache {
        writeln!(file, "n = {}, result = {}", key, value)?;
    }

    Ok(())
}

fn start_collatz_test(n: u64, output_file: &mut File, cache: &mut Cache) -> io::Result<()> {
    for i in 1..=n {
        // Запускаем таймер
        let start_time = Instant::now();
        
        // Выполняем вычисление
        let result = collatz(i, cache);  // Используем кэш
        
        // Останавливаем таймер
        let duration = start_time.elapsed();

        writeln!(output_file, "Collatz({}) = {}, Time taken: {:?}", i, result, duration)?;
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    let n = 100; // Можно изменить значение для тестирования
    
    // Создаем хеш-таблицу (кэш)
    let mut cache = Cache::new();
    
    // Загружаем кэш из файла
    let cache_file = "collatz_cache.txt";
    if Path::new(cache_file).exists() {
        load_cache_from_file(cache_file, &mut cache)?;
    }
    
    // Создаем файл для записи новых результатов
    let mut output_file = File::create("collatz_output.txt")?;
    
    // Запускаем тесты
    start_collatz_test(n, &mut output_file, &mut cache)?;
    
    // Сохраняем обновленный кэш в файл
    save_cache_to_file(cache_file, &cache)?;

    Ok(())
}