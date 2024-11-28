fn return_cars (cars: Vec<i32>, max_weigth: i32 ) -> i32 {
    //position of first car on the bridge
    let mut car1=0;
    //position of second car on the bridge. Assume there are always at least 2 cars
    let mut car2=0;

    let mut skipped_cars =0;

    //Iterate until we are done. Every iteration of the loop executes one check or action.
    while car1 < cars.len() && car2 < cars.len(){
        //make sure car2 is later in line
        if car2 <= car1 {
            car2 +=1;
            continue;
        }

        //This car alone is too heavy
        if cars[car1] > max_weigth {
            car1+=1;
            continue;
        }

        //This car alone is too heavy
        if cars[car2] > max_weigth {
            car2+=1;
            continue;
        }

        //

        if cars[car1] + cars[car2] > max_weigth {
            skipped_cars +=1;

            //Remove the heaviest one
            if cars[car1] > cars[car2] {
                car1+=1;
            }
            else {
                car2+=1;
            }
            continue;
        }

        //car 1 leaves the bridge
        car1 = car2;
    }
    return skipped_cars;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let cars = vec![5,3,8,1,8,7,7,8];
        let result = return_cars(cars, 9);
        assert_eq!(result, 4);
    }
}
