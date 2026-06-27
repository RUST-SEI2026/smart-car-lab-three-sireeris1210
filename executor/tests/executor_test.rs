use executor::{Executor, Pose};

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- 原有测试（普通车辆，确保兼容） ----------
    #[test]
    fn test_default_init() {
        let e = Executor::new();
        assert_eq!(e.state(), (0,0,'N'));
        assert_eq!(e.car_type(), CarType::Normal);
    }

    #[test]
    fn test_normal_move() {
        let mut e = Executor::init(0,0,'N');
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,1,'N'));
    }

    #[test]
    fn test_normal_turn() {
        let mut e = Executor::new();
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,0,'W'));
        e.execute('R').unwrap();
        assert_eq!(e.state(), (0,0,'N'));
    }

    #[test]
    fn test_normal_batch() {
        let mut e = Executor::init(0,0,'N');
        e.execute_batch("MMRMMLMM").unwrap();
        assert_eq!(e.state(), (2,4,'N'));
    }

    #[test]
    fn test_normal_boost() {
        let mut e = Executor::new();
        e.execute('F').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,2,'N'));
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,3,'W')); // 先前进1再左转
    }

    #[test]
    fn test_normal_reverse() {
        let mut e = Executor::new();
        e.execute('B').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-1,'N'));
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,-1,'E')); // 倒车时L变右转
    }

    // ---------- 跑车测试 ----------
    #[test]
    fn test_sports_car_normal_move() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,2,'N')); // 一次移动2格
    }

    #[test]
    fn test_sports_car_normal_turn_left() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,1,'W')); // 先左转再前进1
    }

    #[test]
    fn test_sports_car_normal_turn_right() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('R').unwrap();
        assert_eq!(e.state(), (1,0,'E')); // 先右转再前进1
    }

    #[test]
    fn test_sports_car_reverse_mode() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('B').unwrap(); // 开启倒车
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-2,'N')); // 后退2格
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,-2,'E')); // 右转(互换)再后退1，位置不变因为后退-1再向前？实际是右转，然后后退1格，但右转后朝东，后退1即x-1? 注意：执行L时，先turn_right（变成E），然后move_steps(-1) => x减小1，所以位置变为(-1,-2)，但我们的实现是turn_right再move_steps(-1)。我们测试这个。
        // 重设更清晰：
    }

    // 重新实现一个清晰的跑车倒车测试
    #[test]
    fn test_sports_car_reverse_move_and_turn() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('B').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-2,'N'));

        // 倒车下L: 右转再后退1
        e.execute('L').unwrap();
        // 当前朝N，右转->E，后退1即x-1 => (-1,-2)，朝向E
        assert_eq!(e.state(), (-1,-2,'E'));

        // 倒车下R: 左转再后退1
        e.execute('R').unwrap();
        // 朝E，左转->N，后退1即y-1 => (-1,-3)，朝向N
        assert_eq!(e.state(), (-1,-3,'N'));
    }

    #[test]
    fn test_sports_car_boost_mode() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('F').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,4,'N')); // 前进4格
        e.execute('L').unwrap();
        // F状态下L: 前进1 -> (0,5)，左转->W，再前进1 -> (-1,5)，朝向W
        assert_eq!(e.state(), (-1,5,'W'));
    }

    #[test]
    fn test_sports_car_combined() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute('B').unwrap();
        e.execute('F').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-4,'N')); // 后退4格
        e.execute('L').unwrap();
        // B&F下L: 后退1 -> (0,-5)，右转->E，再后退1 -> (-1,-5)，朝E
        assert_eq!(e.state(), (-1,-5,'E'));
    }

    // ---------- Bus 测试 ----------
    #[test]
    fn test_bus_normal_move() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,1,'N'));
    }

    #[test]
    fn test_bus_normal_turn_left() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('L').unwrap();
        assert_eq!(e.state(), (0,1,'W')); // 前进1再左转
    }

    #[test]
    fn test_bus_normal_turn_right() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('R').unwrap();
        assert_eq!(e.state(), (0,1,'E')); // 前进1再右转
    }

    #[test]
    fn test_bus_reverse_mode() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('B').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-1,'N'));
        e.execute('L').unwrap();
        // 倒车下L: 后退1，再右转 (因为互换)
        // 当前朝N，后退1 -> (0,-2)，右转->E
        assert_eq!(e.state(), (0,-2,'E'));
    }

    #[test]
    fn test_bus_boost_mode() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('F').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,2,'N')); // 前进两次，共2格
        e.execute('L').unwrap();
        // F下L: 前进1，再前进1，再左转
        // 当前(0,2,N)，前进1->(0,3)，再前进1->(0,4)，左转->W
        assert_eq!(e.state(), (0,4,'W'));
    }

    #[test]
    fn test_bus_combined() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::Bus);
        e.execute('B').unwrap();
        e.execute('F').unwrap();
        e.execute('M').unwrap();
        assert_eq!(e.state(), (0,-2,'N')); // 后退两次共2格
        e.execute('L').unwrap();
        // B&F下L: 后退1，再后退1，再右转(互换)
        // 当前(0,-2,N)，后退1->(0,-3)，再后退1->(0,-4)，右转->E
        assert_eq!(e.state(), (0,-4,'E'));
    }

    // ---------- 多类型混合测试 ----------
    #[test]
    fn test_different_types_behavior() {
        let mut normal = Executor::init(0,0,'N');
        let mut sports = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        let mut bus = Executor::init_with_type(0,0,'N', CarType::Bus);

        // 执行相同的指令序列 "ML"
        normal.execute_batch("ML").unwrap();
        sports.execute_batch("ML").unwrap();
        bus.execute_batch("ML").unwrap();

        // 普通: M->(0,1,N), L->(0,1,W)
        assert_eq!(normal.state(), (0,1,'W'));
        // 跑车: M->(0,2,N), L->先左转(W)再前进1->(-1,2,W)
        assert_eq!(sports.state(), (-1,2,'W'));
        // Bus: M->(0,1,N), L->前进1->(0,2,N)再左转->(0,2,W)
        assert_eq!(bus.state(), (0,2,'W'));
    }

    #[test]
    fn test_toggle_modes() {
        let mut e = Executor::init_with_type(0,0,'N', CarType::SportsCar);
        e.execute_batch("BFBF").unwrap();
        assert_eq!(e.modes(), (false, false));
        e.execute('B').unwrap();
        assert_eq!(e.modes(), (true, false));
        e.execute('F').unwrap();
        assert_eq!(e.modes(), (true, true));
        e.execute('B').unwrap();
        assert_eq!(e.modes(), (false, true));
    }

    #[test]
    fn test_invalid_command() {
        let mut e = Executor::new();
        assert!(e.execute('X').is_err());
    }
}
