#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heading {
    N, S, E, W,
}

impl Heading {
    pub fn turn_left(&self) -> Self {
        match self {
            Heading::N => Heading::W,
            Heading::W => Heading::S,
            Heading::S => Heading::E,
            Heading::E => Heading::N,
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N,
        }
    }
}

impl From<char> for Heading {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => Heading::N,
            'S' => Heading::S,
            'E' => Heading::E,
            'W' => Heading::W,
            _ => panic!("无效的朝向字符，只能是 N, S, E, W"),
        }
    }
}

/// 车辆类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarType {
    Normal,     // 普通车辆（默认）
    SportsCar,  // 跑车
    Bus,        // Bus
}

/// 执行器组件
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executor {
    x: i32,
    y: i32,
    heading: Heading,
    reverse_mode: bool,
    boost_mode: bool,
    car_type: CarType,
}

impl Executor {
    // ---------- 构造器 ----------
    /// 默认普通车辆，位置(0,0)，朝向N
    pub fn new() -> Self {
        Self::default()
    }

    /// 普通车辆自定义初始化（保持向后兼容）
    pub fn init(x: i32, y: i32, heading: char) -> Self {
        Self::init_with_type(x, y, heading, CarType::Normal)
    }

    /// 自定义车辆类型初始化
    pub fn init_with_type(x: i32, y: i32, heading: char, car_type: CarType) -> Self {
        Executor {
            x,
            y,
            heading: Heading::from(heading),
            reverse_mode: false,
            boost_mode: false,
            car_type,
        }
    }

    // ---------- 指令执行 ----------
    pub fn execute(&mut self, command: char) -> Result<(), &'static str> {
        match command {
            'M' => self.exec_move(),
            'L' => self.exec_turn_left(),
            'R' => self.exec_turn_right(),
            'B' => { self.reverse_mode = !self.reverse_mode; Ok(()) }
            'F' => { self.boost_mode = !self.boost_mode; Ok(()) }
            _ => Err("无效指令，只支持 M, L, R, B, F"),
        }
    }

    pub fn execute_batch(&mut self, commands: &str) -> Result<(), &'static str> {
        for c in commands.chars() {
            self.execute(c)?;
        }
        Ok(())
    }

    // ---------- 内部指令实现 ----------
    fn exec_move(&mut self) -> Result<(), &'static str> {
        let steps = match (self.car_type, self.reverse_mode, self.boost_mode) {
            (CarType::Normal, false, false) => 1,
            (CarType::Normal, true, false) => -1,
            (CarType::Normal, false, true) => 2,
            (CarType::Normal, true, true) => -2,
            (CarType::SportsCar, false, false) => 2,
            (CarType::SportsCar, true, false) => -2,
            (CarType::SportsCar, false, true) => 4,
            (CarType::SportsCar, true, true) => -4,
            (CarType::Bus, false, false) => 1,
            (CarType::Bus, true, false) => -1,
            (CarType::Bus, false, true) => { self.move_steps(1); self.move_steps(1); return Ok(()); }
            (CarType::Bus, true, true) => { self.move_steps(-1); self.move_steps(-1); return Ok(()); }
        };
        self.move_steps(steps);
        Ok(())
    }

    fn exec_turn_left(&mut self) -> Result<(), &'static str> {
        match (self.car_type, self.reverse_mode, self.boost_mode) {
            // Normal
            (CarType::Normal, false, false) => self.turn_left(),
            (CarType::Normal, true, false) => self.turn_right(),
            (CarType::Normal, false, true) => { self.move_steps(1); self.turn_left(); }
            (CarType::Normal, true, true) => { self.move_steps(-1); self.turn_right(); }
            // SportsCar
            (CarType::SportsCar, false, false) => { self.turn_left(); self.move_steps(1); }
            (CarType::SportsCar, true, false) => { self.turn_right(); self.move_steps(-1); }
            (CarType::SportsCar, false, true) => { self.move_steps(1); self.turn_left(); self.move_steps(1); }
            (CarType::SportsCar, true, true) => { self.move_steps(-1); self.turn_right(); self.move_steps(-1); }
            // Bus
            (CarType::Bus, false, false) => { self.move_steps(1); self.turn_left(); }
            (CarType::Bus, true, false) => { self.move_steps(-1); self.turn_right(); }
            (CarType::Bus, false, true) => { self.move_steps(1); self.move_steps(1); self.turn_left(); }
            (CarType::Bus, true, true) => { self.move_steps(-1); self.move_steps(-1); self.turn_right(); }
        }
        Ok(())
    }

    fn exec_turn_right(&mut self) -> Result<(), &'static str> {
        match (self.car_type, self.reverse_mode, self.boost_mode) {
            // Normal
            (CarType::Normal, false, false) => self.turn_right(),
            (CarType::Normal, true, false) => self.turn_left(),
            (CarType::Normal, false, true) => { self.move_steps(1); self.turn_right(); }
            (CarType::Normal, true, true) => { self.move_steps(-1); self.turn_left(); }
            // SportsCar
            (CarType::SportsCar, false, false) => { self.turn_right(); self.move_steps(1); }
            (CarType::SportsCar, true, false) => { self.turn_left(); self.move_steps(-1); }
            (CarType::SportsCar, false, true) => { self.move_steps(1); self.turn_right(); self.move_steps(1); }
            (CarType::SportsCar, true, true) => { self.move_steps(-1); self.turn_left(); self.move_steps(-1); }
            // Bus
            (CarType::Bus, false, false) => { self.move_steps(1); self.turn_right(); }
            (CarType::Bus, true, false) => { self.move_steps(-1); self.turn_left(); }
            (CarType::Bus, false, true) => { self.move_steps(1); self.move_steps(1); self.turn_right(); }
            (CarType::Bus, true, true) => { self.move_steps(-1); self.move_steps(-1); self.turn_left(); }
        }
        Ok(())
    }

    // ---------- 辅助函数 ----------
    fn move_steps(&mut self, steps: i32) {
        match self.heading {
            Heading::N => self.y += steps,
            Heading::S => self.y -= steps,
            Heading::E => self.x += steps,
            Heading::W => self.x -= steps,
        }
    }
    fn turn_left(&mut self) { self.heading = self.heading.turn_left(); }
    fn turn_right(&mut self) { self.heading = self.heading.turn_right(); }

    // ---------- 查询接口 ----------
    pub fn position(&self) -> (i32, i32) { (self.x, self.y) }
    pub fn heading(&self) -> Heading { self.heading }
    pub fn heading_char(&self) -> char {
        match self.heading {
            Heading::N => 'N',
            Heading::S => 'S',
            Heading::E => 'E',
            Heading::W => 'W',
        }
    }
    pub fn state(&self) -> (i32, i32, char) {
        (self.x, self.y, self.heading_char())
    }
    pub fn modes(&self) -> (bool, bool) { (self.reverse_mode, self.boost_mode) }
    pub fn car_type(&self) -> CarType { self.car_type }
}

impl Default for Executor {
    fn default() -> Self {
        Executor {
            x: 0,
            y: 0,
            heading: Heading::N,
            reverse_mode: false,
            boost_mode: false,
            car_type: CarType::Normal,
        }
    }
}
