/*mod search {
    // --- Función de búsqueda binaria ---
    pub fn bin_search(t: i32, a: &[i32]) -> isize {
        if a.len() == 0 {
            return -1;
        }

        let mut inicio = 0;
        let mut fin = a.len() - 1;

        while inicio <= fin {
            let medio = (inicio + fin) / 2;
            let valor = a[medio];

            if t == valor {
                return medio as isize;
            } else if t < valor {
                if medio == 0 {
                    return -1;
                }
                fin = medio - 1;
            } else {
                inicio = medio + 1;
            }
        }

        -1
    }
}

use search::bin_search;

fn count(a: &mut [i32]) -> u32 {
    // Ordenamos el array para poder usar búsqueda binaria
    a.sort();

    let mut contador: u32 = 0;

    for i in 0..a.len() {
        let valor = a[i];
        let buscado = -valor;

        // Buscamos solo en la parte derecha (evita contar parejas dos veces)
        if let Some(subarray) = a.get(i + 1..) {
            if bin_search(buscado, subarray) != -1 {
                contador += 1;
            }
        }
    }

    contador
}

#[test]
fn test_count() {
    assert_eq!(count(&mut [0, 1, 3]), 0);
    assert_eq!(count(&mut [-1, 1, 2]), 1);
    assert_eq!(count(&mut [-1, 2, 1, -2]), 2);
    assert_eq!(count(&mut [7, -1, 4, 5, 22, -7, 2, 1, -2]), 3);
}

fn main() {
    println!("{}", count(&mut [-1, 1, 2])); // Ejemplo: imprime 1
}
*/

fn bin_search(t: i32, a: &[i32]) -> isize {
    //Comprobamos array
    if a.len()== 0{
        return -1;
    }

    //Limites
    let mut inicio = 0;
    let mut fin = a.len() - 1;

    //Recorremos el array a
    while inicio <= fin {

        // Calculamos el centro del array
        let medio = (inicio + fin)/2;

        // Accedemos al valor del medio del array
        let valor = a[medio];

        //Comprobamos el valor de t
        if t == valor{
            return medio as isize;
        } else if t < valor{
            if medio == 0{
                return -1;
            }
            //Reducimos el valor final
            fin= medio -1;
        } else {
            //Aumentamos el valor inicial
            inicio = medio +1;
            
        }

    }
    return -1; //Si no esta, devolvemos -1
}

fn count(a: &mut [i32]) -> u32 {
    // Ordenamos el array (la búsqueda binaria requiere que esté ordenado)
    a.sort();

    let mut contador = 0;

    // Recorremos cada elemento del array
    for i in 0..a.len() {
        let valor = a[i];
        let opuesto = -valor;

        // Buscamos el opuesto solo en la parte posterior del array
        if bin_search(opuesto, &a[i + 1..]) != -1 {
            contador += 1;
        }
    }

    contador
}

#[test]
fn test_count() {
    assert_eq!(count(&mut [0, 1, 3]), 0);
    assert_eq!(count(&mut [-1, 1, 2]), 1);
    assert_eq!(count(&mut [-1, 2, 1, -2]), 2);
    assert_eq!(count(&mut [7, -1, 4, 5, 22, -7, 2, 1, -2]), 3);
}
