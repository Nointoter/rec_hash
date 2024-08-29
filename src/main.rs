use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
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

fn start_collatz_test(n: u64, file: &mut File) -> io::Result<()> {
    let mut cache = Cache::new();
    
    for i in 1..=n {
        // Запускаем таймер
        let start_time = Instant::now();
        
        // Выполняем вычисление
        let result = collatz(i, &mut cache);  // Изменено с n на i
        
        // Останавливаем таймер
        let duration = start_time.elapsed();

        writeln!(file, "Collatz({}) = {}, Time taken: {:?}", i, result, duration)?; // Записываем результат в файл
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    let n = 100; // Можно изменить значение для тестирования
    
    // Создаем файл для записи и обрабатываем возможные ошибки
    let mut file = File::create("collatz_output.txt")?;
    
    // Запускаем тесты
    start_collatz_test(n, &mut file)?;

    Ok(())
}