use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

// Определяем тип для кэша
type Cache = HashMap<u64, u64>;

// Определяем структуру для кэша
#[derive(Serialize, Deserialize, Clone)]
struct CacheWrapper {
    cache: Cache,
}

// Определяем структуру для результата
#[derive(Serialize, Deserialize)]
struct ResultEntry {
    input: u64,
    result: u64,
    time_taken: String,
}

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

fn load_cache_from_file(filename: &str) -> io::Result<Cache> {
    if Path::new(filename).exists() {
        let file = File::open(filename)?;
        let cache_wrapper: CacheWrapper = serde_json::from_reader(file)?;
        Ok(cache_wrapper.cache)
    } else {
        Ok(HashMap::new())
    }
}

fn save_cache_to_file(filename: &str, cache: &Cache) -> io::Result<()> {
    let cache_wrapper = CacheWrapper {
        cache: cache.clone(),
    };
    let file = File::create(filename)?;
    serde_json::to_writer(file, &cache_wrapper)?;
    Ok(())
}

fn start_collatz_test(n: u64, results: &mut Vec<ResultEntry>, cache: &mut Cache) -> io::Result<()> {
    for i in 1..=n {
        // Запускаем таймер
        let start_time = Instant::now();
        
        // Выполняем вычисление
        let result = collatz(i, cache);  // Используем кэш
        
        // Останавливаем таймер
        let duration = start_time.elapsed();
        let duration_str = format!("{:?}", duration);

        // Добавляем запись в результаты
        results.push(ResultEntry {
            input: i,
            result,
            time_taken: duration_str,
        });
    }
    
    Ok(())
}

fn save_results_to_file(filename: &str, results: &[ResultEntry]) -> io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, results)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let n = 10000; // Можно изменить значение для тестирования

    // Создаем хеш-таблицу (кэш)
    let cache_file = "cache/collatz_cache.json";
    let results_file = "results/collatz_output.json";

    // Проверяем и создаем директории, если их нет
    let cache_dir = "cache";
    let results_dir = "results";

    if !Path::new(cache_dir).exists() {
        fs::create_dir(cache_dir)?;
    }
    
    if !Path::new(results_dir).exists() {
        fs::create_dir(results_dir)?;
    }

    // Загружаем кэш из файла, если файл существует
    let mut cache = load_cache_from_file(cache_file)?;

    // Создаем вектор для записи результатов
    let mut results = Vec::new();

    // Запускаем тесты
    start_collatz_test(n, &mut results, &mut cache)?;

    // Сохраняем обновленный кэш в файл
    save_cache_to_file(cache_file, &cache)?;

    // Сохраняем результаты в JSON-файл
    save_results_to_file(results_file, &results)?;

    Ok(())
}
