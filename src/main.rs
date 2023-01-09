use std::io;
use std::process::exit;


const CERO: &str = "CERO";

const CIEN: &str = "CIEN";

const MIL: &str = "MIL";

const NUMEROS: [&str; 31] = ["",
    "UN", "DOS", "TRES", "CUATRO", "CINCO", "SEIS", "SIETE", "OCHO", "NUEVE", "DIEZ",
    "ONCE", "DOCE", "TRECE", "CATORCE", "QUINCE", "DIECISÉIS", "DIECISIETE", "DIECIOCHO",
    "DIECINUEVE", "VEINTE", "VEINTIUN", "VEINTIDOS", "VEINTITRÉS", "VEINTICUATRO",
    "VEINTICINCO", "VEINTISÉIS", "VEINTISIETE", "VEINTIOCHO", "VEINTINUEVE", "TREINTA"
];

const DECENAS: [&str; 10] = [ "",
    "DIEZ", "VEINTE", "TREINTA", "CUARENTA", "CINCUENTA",
    "SESENTA", "SETENTA", "OCHENTA", "NOVENTA"
];

const CENTENAS: [&str; 10] = [ "",
    "CIENTO", "DOSCIENTOS", "TRESCIENTOS", "CUATROCIENTOS", "QUINIENTOS",
    "SEISCIENTOS", "SETECIENTOS", "OCHOCIENTOS", "NOVECIENTOS"
];

const MILLONES: [&str; 2] = ["MILLÓN", "MILLONES"];

const BILLONES: [&str; 2] = ["BILLÓN", "BILLONES"];

const TRILLONES: [&str; 2] = ["TRILLÓN", "TRILLONES"];

const CUATRILLONES: [&str; 2] = ["CUATRILLÓN", "CUATRILLONES"];

const QUINTIILLONES: [&str; 2] = ["QUINTILLÓN", "QUINTILLONES"];


fn entrada(mensaje: &str) -> String {
    let mut captura = String::new();
    println!("{}", mensaje);
    io::stdin().read_line(&mut captura).expect("No se pudo leer la entrada");
    return String::from(captura.trim());
}

fn terminar_programa(){
    let salir = entrada("Para salir escriba 'salir' o presione enter para continuar.");
    if salir == "salir" {
        println!("Programa finalizado.");
        exit(0);
    }
}

fn numero_a_texto_decimos(numero:i128) -> String{
    let numero_sin_signo: usize = numero as usize;

    if 0 <= numero && numero <= 30 {
       return NUMEROS[numero_sin_signo].to_string();
    }
    if  NUMEROS[numero_sin_signo%10] != ""{
        return DECENAS[numero_sin_signo/10].to_string() + " Y " + NUMEROS[numero_sin_signo%10];
    }
    return DECENAS[numero_sin_signo/10].to_string();
}

fn numero_a_texto_cientos(numero:i128) -> String{
    if numero < 100{
        return numero_a_texto_decimos(numero);
    }

    let numero_sin_signo: usize = numero as usize;
    if numero == 100{
        return CIEN.to_string();
    }

    if CENTENAS[numero_sin_signo/100] != ""{
        return CENTENAS[numero_sin_signo/100].to_string()
            + " "
            + numero_a_texto_decimos(numero%100).as_str();
    }

    return numero_a_texto_decimos(numero%100);
}

fn numero_a_texto_miles(numero:i128) -> String{
    let mil = 1000;
    if numero < mil{
        return numero_a_texto_cientos(numero);
    }

    if numero == mil{
        return MIL.to_string();
    }

    if numero/mil == 1 {
        return MIL.to_string() + " " + numero_a_texto_cientos(numero % mil).as_str()
    }
    return numero_a_texto_cientos(numero/mil)
        + " " + MIL +" "
        + numero_a_texto_cientos(numero % mil).as_str()
}

fn numero_a_texto_millones(numero:i128) -> String{
    let millon = 1000000;
    if numero < millon{
        return numero_a_texto_miles(numero);
    }

    let mut resto= String::from("");
    if numero % millon != 0 {
        resto = resto + &numero_a_texto_miles(numero % millon);
    }

    if resto == ""{
        if numero/millon == 1 {
            return NUMEROS[1].to_string()
                + " " + MILLONES[0] + " DE"
                + resto.as_str();
        }

        return numero_a_texto_miles(numero/millon)
            + " " + MILLONES[1] +" DE";
    }

    if numero/millon == 1 {
        return NUMEROS[1].to_string()
            + " " + MILLONES[0] + " "
            + resto.as_str();
    }

    return numero_a_texto_miles(numero/millon)
        + " " + MILLONES[1] +" "
        + resto.as_str();
}

fn numero_a_texto_billones(numero:i128) -> String{
    let billon = 1000000000000;
    if numero < billon{
        return numero_a_texto_millones(numero);
    }

    let mut resto= String::from("");
    if numero % billon != 0 {
        resto = resto + &numero_a_texto_millones(numero % billon);
    }

    if resto == ""{
        if numero/billon == 1 {
            return NUMEROS[1].to_string()
                + " " + BILLONES[0] + " DE"
                + resto.as_str();
        }

        return numero_a_texto_miles(numero/billon)
            + " " + BILLONES[1] +" DE";
    }

    if numero/billon == 1 {
        return NUMEROS[1].to_string()
            + " " + BILLONES[0] + " "
            + resto.as_str();
    }

    return numero_a_texto_miles(numero/billon)
        + " " + BILLONES[1] +" "
        + resto.as_str();
}

fn numero_a_texto_trillones(numero:i128) -> String{
    let trillon = 1000000000000000000;
    if numero < trillon{
        return numero_a_texto_billones(numero);
    }

    let mut resto= String::from("");
    if numero % trillon != 0 {
        resto = resto + &numero_a_texto_billones(numero % trillon);
    }

    if resto == ""{
        if numero/trillon == 1 {
            return NUMEROS[1].to_string()
                + " " + TRILLONES[0] + " DE"
                + resto.as_str();
        }

        return numero_a_texto_miles(numero/trillon)
            + " " + TRILLONES[1] +" DE";
    }

    if numero/trillon == 1 {
        return NUMEROS[1].to_string()
            + " " + TRILLONES[0] + " "
            + resto.as_str();
    }

    return numero_a_texto_miles(numero/trillon)
        + " " + TRILLONES[1] +" "
        + resto.as_str();
}

fn numero_a_texto_cuatrillones(numero:i128) -> String{
    let cuatrillon = 1000000000000000000000000;
    if numero < cuatrillon{
        return numero_a_texto_trillones(numero);
    }

    let mut resto= String::from("");
    if numero % cuatrillon != 0 {
        resto = resto + &numero_a_texto_trillones(numero % cuatrillon);
    }

    if resto == ""{
        if numero/cuatrillon == 1 {
            return NUMEROS[1].to_string()
                + " " + CUATRILLONES[0] + " DE"
                + resto.as_str();
        }

        return numero_a_texto_miles(numero/cuatrillon)
            + " " + CUATRILLONES[1] +" DE";
    }

    if numero/cuatrillon == 1 {
        return NUMEROS[1].to_string()
            + " " + CUATRILLONES[0] + " "
            + resto.as_str();
    }

    return numero_a_texto_miles(numero/cuatrillon)
        + " " + CUATRILLONES[1] +" "
        + resto.as_str();
}

fn numero_a_texto_quintillones(numero:i128) -> String{
    let quintillon = 1000000000000000000000000000000;
    if numero < quintillon{
        return numero_a_texto_cuatrillones(numero);
    }

    let mut resto= String::from("");
    if numero % quintillon != 0 {
        resto = resto + &numero_a_texto_cuatrillones(numero % quintillon);
    }

    if resto == ""{
        if numero/quintillon == 1 {
            return NUMEROS[1].to_string()
                + " " + QUINTIILLONES[0] + " DE"
                + resto.as_str();
        }

        return numero_a_texto_miles(numero/quintillon)
            + " " + QUINTIILLONES[1] +" DE";
    }

    if numero/quintillon == 1 {
        return NUMEROS[1].to_string()
            + " " + QUINTIILLONES[0] + " "
            + resto.as_str();
    }

    return numero_a_texto_miles(numero/quintillon)
        + " " + QUINTIILLONES[1] +" "
        + resto.as_str();
}

fn main() {


    loop{
        println!("******** Programa de Monto Escrito ********");
        //Capturar el número como String
        let mut number_as_string = entrada("Escriba un número: ");

        // Validar que el numero sea valido y quitar espacios
        let number_as_integer:i128 = number_as_string.parse().unwrap_or(-1);
        if number_as_integer == -1 {
            println!("Número inválido");
            terminar_programa();
            continue;
        }

        number_as_string = number_as_integer.to_string();

        println!("Número ingresado: {} ", number_as_string);

        if number_as_integer < 0{
            println!("No se aceptan números negativos");
            terminar_programa();
            continue;
        }

        if number_as_integer == 0{
            println!("{} PESOS", CERO);
            terminar_programa();
            continue;
        }

        if number_as_integer == 1{
            println!("{} PESO", numero_a_texto_decimos(number_as_integer));
            terminar_programa();
            continue;
        }

        if number_as_integer <= 9999999999999999999999999999999{
            println!("{} PESOS", numero_a_texto_quintillones(number_as_integer));
            terminar_programa();
            continue;
        }

        println!("Número supera los quintillones, aún no esta implementado soporte para más.");
    }
}
