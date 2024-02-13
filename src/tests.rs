    use super::*;
    use serde_json::json;
    #[test]
    fn test_spring_no_dst_curve() {
        let date = Date { day: 1, month: 3}; // Spring date without DST
        let season = get_season(&date);
        let dst = is_dst(&date);
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);
        // Define the expected spring curve without DST
        let expected_curve = json!({
            "morning": {"temp": [2500,3500],"brightness": [20,70],"time": [6,8]},
            "daytime" : {"temp": [5500,6500],"brightness": [70,100],"time": [8,18]},
            "evening": {"temp": [4000,3000],"brightness": [70,40],"time": [18,20]},
            "night": {"temp": [2700,2000],"brightness": [40,10],"time": [20,22]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [22,6]}
        });
        assert_eq!(curve, expected_curve);
    }

    #[test]
    fn test_spring_with_dst_curve() {
        let date = Date { day: 15, month: 4 }; // Spring date with DST
        let season = get_season(&date);
        let dst = true; // Assuming DST is true for this test
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);
        // Define the expected spring curve with DST
        let expected_curve = json!({
            "morning": {"temp": [2500,3500],"brightness": [20,70],"time": [7,9]},
            "daytime" : {"temp": [5500,6500],"brightness": [70,100],"time": [9,19]},
            "evening": {"temp": [4000,3000],"brightness": [70,40],"time": [19,21]},
            "night": {"temp": [2700,2000],"brightness": [40,10],"time": [21,23]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [23,7]}
        });

        assert_eq!(curve, expected_curve);
    }
    
    #[test]
    fn test_fall_no_dst_curve() {
        let date = Date { day: 10, month: 11 }; // Spring date without DST
        let season = get_season(&date);
        let dst = is_dst(&date);
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);
        // Define the expected spring curve without DST
        let expected_curve = json!({
            "morning": {"temp": [2500, 3500],"brightness": [20, 70],"time": [6, 8]},
            "daytime": {"temp": [5500, 6500],"brightness": [70, 100],"time": [8, 18]},
            "evening": {"temp": [4000, 3000],"brightness": [70, 30],"time": [18, 20]},
            "night": {"temp": [2700, 2000],"brightness": [30, 10],"time": [20, 22]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [22,6]}
        });

        assert_eq!(curve, expected_curve);
    }

    #[test]
    fn test_fall_with_dst_curve() {
        let date = Date { day: 10, month: 10 }; // Spring date with DST
        let season = get_season(&date);
        let dst = true; // Assuming DST is true for this test
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);

        // Define the expected spring curve with DST
        let expected_curve = json!({
            "morning": {"temp": [2500, 3500],"brightness": [20, 70],"time": [7,9]},
            "daytime": {"temp": [5500, 6500],"brightness": [70, 100],"time": [9,19]},
            "evening": {"temp": [4000, 3000],"brightness": [70, 30],"time": [19,21]},
            "night": {"temp": [2700, 2000],"brightness": [30, 10],"time": [21,23]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [23,7]}
        });

        assert_eq!(curve, expected_curve);
    }

    #[test]
    fn test_summer_curve() {
        let date = Date { day: 7, month: 7 }; // Spring date with DST
        let season = get_season(&date);
        let dst = true; // Assuming DST is true for this test
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);

        // Define the expected spring curve with DST
        let expected_curve = json!({
            "morning": {"temp": [3000,5000],"brightness": [30,100],"time": [7,9]},
            "daytime" : {"temp": [6500,6500],"brightness": [100,100],"time": [9,19]},
            "evening": {"temp": [4000,3000],"brightness": [100,50],"time": [19,22]},
            "night": {"temp": [2700,2200],"brightness": [50,5],"time": [22,23]},
            "nocturn": {"temp": [2200,2200],"brightness": [5,5],"time": [23,7]}
        });

        assert_eq!(curve, expected_curve);
    }

    #[test]
    fn test_winter_curve() {
        let date = Date { day: 1, month: 1}; // Spring date with DST
        let season = get_season(&date);
        let dst = true; // Assuming DST is true for this test
        let curve = select_curve(&season, dst);
        let curve = json!(*curve);

        // Define the expected spring curve with DST
        let expected_curve = json!({
            "morning": {"temp": [2000,3000], "brightness": [10,60], "time": [6,8]},
            "daytime": {"temp": [5000,6500], "brightness": [60,100], "time": [8,16]},
            "evening": {"temp": [4000,3000], "brightness": [60,30], "time": [16,20]},
            "night": {"temp": [2700,1800], "brightness": [30,5], "time": [20,22]},
            "nocturn": {"temp": [1800,1800], "brightness": [5,5], "time": [22,6]},
        });

        assert_eq!(curve, expected_curve);
    }
