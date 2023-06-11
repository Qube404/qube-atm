fn main() {
    
}

fn atm(amount: u32) -> Result<[u32; 2], ()> {
    // If amount is between 0 and 20 return an error
    if amount > 0 && amount < 20 {
        return Err(());
    }

    return Ok([amount / 50 - ((amount % 50 % 20) / 10), (amount % 50 + (amount % 50 % 20) * 5) / 20]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_dollars() {
        let amount = 0;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [0, 0]);
    }

    #[test]
    fn test_10_dollars() {
        let amount = 10;
        if let Err(result) = atm(amount) {
            assert_eq!(result, ());
        }
    }

    #[test]
    fn test_20_dollars() {
        let amount = 20;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [0, 1]);
    }


    #[test]
    fn test_200_dollars() {
        let amount = 200;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [4, 0]);
    }

    #[test]
    fn test_220_dollars() {
        let amount = 220;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [4, 1]);
    }

    #[test]
    fn test_230_dollars() {
        let amount = 230;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [3, 4]);
    }

    #[test]
    fn test_235_dollars() {
        let amount = 235;
        let result = atm(amount);

        if let Err(result) = result {
            assert_eq!(result, ());
        }
    }

    #[test]
    fn test_240_dollars() {
        let amount = 240;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [4, 2]);
    }

    #[test]
    fn test_290_dollars() {
        let amount = 290;
        let result = atm(amount)
            .unwrap();

        assert_eq!(result, [5, 2]);
    }
}
