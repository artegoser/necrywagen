#[macro_export]
macro_rules! pre_print {
    ($k:expr,$start:expr, $Locale:expr) => {
        if $k % 100000 == 0 || $k == 1 {
            crate::print_force!($k, $start, $Locale);
        }
        print!("\r");
    };
}

#[macro_export]
macro_rules! print_force {
    ($k:expr, $start:expr , $Locale:expr) => {
        let duration = $start.elapsed();

        let persec = ($k as f64 / duration.as_secs_f64()) as u64;

        print!(
            " {} keys; {} keys/sec; {:?}\t\t",
            $k.to_formatted_string(&$Locale),
            persec.to_formatted_string(&$Locale),
            duration
        );
    };
}

#[macro_export]
macro_rules! print_progress {
    ($k_mutex:expr, $start:expr , $Locale:expr) => {
        crate::not_mutex_print_progress!(*$k_mutex.lock().unwrap(), $start, $Locale);
    };
    ($k_mutex:expr, $start:expr, $Locale:expr, $count:expr) => {
        crate::not_mutex_print_progress!(*$k_mutex.lock().unwrap(), $start, $Locale, $count);
    };
}

#[macro_export]
macro_rules! not_mutex_print_progress {
    ($k:expr, $start:expr , $Locale:expr) => {
        $k += 1;

        crate::pre_print!($k, $start, $Locale);
    };
    ($k:expr, $start:expr, $Locale:expr, $count:expr) => {
        crate::pre_print!($k, $start, $Locale);

        $k += 1;
        if $k >= $count {
            break;
        }
    };
}
