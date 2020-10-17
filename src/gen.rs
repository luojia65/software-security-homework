use rand::seq::SliceRandom;

fn make_fn(ret: &str, name: &str, param: &str, content: &str) -> String {
    let content = {
        let mut ans = String::new();
        for line in content.lines() {
            ans.push_str("    ");
            ans.push_str(line);
            ans.push_str("\n");
        }
        ans
    };
    format!("{} {}({}) {{\n{}}}\n", ret, name, param, content)
}

fn make_var_def(ty: &str, name: &str) -> String {
    format!("{} {};\n", ty, name)
}

fn make_array_var_def(ty: &str, len: &str, name: &str) -> String {
    format!("{}[{}] {};\n", ty, len, name)
}

fn make_fn_param(ty: &str, name: &str) -> String {
    format!("{} {}", ty, name)
}

fn make_var_assign(l: &str, r: &str) -> String {
    format!("{} = {};\n", l, r)
}

fn make_if(condition: &str, content: &str) -> String {
    let content = {
        let mut ans = String::new();
        for line in content.lines() {
            ans.push_str("    ");
            ans.push_str(line);
            ans.push_str("\n");
        }
        ans
    };
    format!("if ({}) {{\n{}}}\n", condition, content)
}

pub fn generate_r4() -> String {
    let mut ans = String::new();
    let mut rng = rand::thread_rng();
    let types = vec!["short", "unsigned short", "int", "unsigned int", "char", "unsigned char", "int*"];
    for _ in 0..5 {
        let mut vars = Vec::new();
        let mut content = String::new();
        let mut param = String::new();
        for i in 0..10 {
            let name = random_ident();
            let ty = types.choose(&mut rng).unwrap();
            let len = rand::random::<usize>() % 1000;
            if i < 2 {
                param.push_str(&make_fn_param(ty, &name));
                if i < 1 { param.push_str(", "); }
            } else {
                content.push_str(&make_array_var_def(ty, &len.to_string(), &name));
            }
            vars.push(name);
        }
        for _ in 0..20 {
            let iter = vars.choose_multiple(&mut rng, 5).collect::<Vec<_>>();
            let r = match rand::random::<usize>() % 2 {
                0 => format!("strcpy({}, \"{}\")", iter[0], random_string()),
                1 => format!("strncpy({}, \"{}\", {})", iter[0], random_string(), rand::random::<usize>() % 100),  
                _ => unreachable!()
            };
            content.push_str(&(r + ";\n"));
        }
        ans += &make_fn("void", &random_ident(), &param, &content)
    }
    ans
}

pub fn generate_r5() -> String {
    let mut rng = rand::thread_rng();
    let types = vec!["short", "unsigned short", "int", "unsigned int", "char", "unsigned char", "int*"];
    let mut vars = Vec::new();
    let mut content = String::new();
    let mut param = String::new();
    for i in 0..10 {
        let name = random_ident();
        let ty = types.choose(&mut rng).unwrap();
        if i < 2 {
            param.push_str(&make_fn_param(ty, &name));
            if i < 1 { param.push_str(", "); }
        } else {
            content.push_str(&make_var_def(ty, &name));
        }
        vars.push(name);
    }
    for _ in 0..20 {
        let iter = vars.choose_multiple(&mut rng, 5).collect::<Vec<_>>();
        let j = {
            let mut a = String::new();
            for _ in 0..rand::random::<usize>() % 5 {
                match rand::random::<usize>() % 4 {
                    0 => a.push_str("%d"),
                    1 => a.push_str("%n"),
                    2 => a.push_str("%lld"),
                    3 => a.push_str("%f"),
                    _ => {}
                }
            }
            a
        };
        let mut r = String::new();
        for _ in 0..rand::random::<usize>() % 5 {
            r.push_str(", ");
            r.push_str(iter[rand::random::<usize>() % 5]);
        }
        let r = format!("printf(\"{}\"{})", j, r);
        content.push_str(&(r + ";\n"));
    }
    make_fn("void", &random_ident(), &param, &content)
}

pub fn generate_b2() -> String {
    let mut rng = rand::thread_rng();
    let types = vec!["short", "unsigned short", "int", "unsigned int", "char", "unsigned char", "int*"];
    let mut vars = Vec::new();
    let mut content = String::new();
    let mut param = String::new();
    for i in 0..10 {
        let name = random_ident();
        let ty = types.choose(&mut rng).unwrap();
        if i < 2 {
            param.push_str(&make_fn_param(ty, &name));
            if i < 1 { param.push_str(", "); }
        } else {
            content.push_str(&make_var_def(ty, &name));
        }
        vars.push(name);
    }
    for _ in 0..20 {
        let iter = vars.choose_multiple(&mut rng, 2).collect::<Vec<_>>();
        content.push_str(&make_var_assign(iter[0], iter[1]));
    }
    make_fn("void", &random_ident(), &param, &content)
}

pub fn generate_b3() -> String {
    let mut rng = rand::thread_rng();
    let types = vec!["short*", "unsigned short*", "int*", "unsigned int*", "char*", "unsigned char*"];
    let types_unref = vec!["short", "int", "char"];
    let mut vars = Vec::new();
    let mut content = String::new();
    let mut param = String::new();
    for i in 0..10 {
        let name = random_ident();
        let ty = *types.choose(&mut rng).unwrap();
        if i < 2 {
            param.push_str(&make_fn_param(ty, &name));
            if i < 1 { param.push_str(", "); }
        } else {
            content.push_str(&make_var_def(ty, &name));
        }
        vars.push((ty, name));
    }
    for _ in 0..20 {
        let (ty, var) = vars.choose(&mut rng).unwrap();
        let iter = vars.choose_multiple(&mut rng, 2).collect::<Vec<_>>();
        let new_ty = *types_unref.choose(&mut rng).unwrap();
        let expr = match rand::random::<usize>() % 5 {
            0 => format!("{}", iter[0].1),
            1 => format!("{} * {}", iter[0].1, iter[1].1),
            2 => format!("{} + {}", iter[0].1, iter[1].1),
            3 => format!("{} * sizeof({})", iter[0].1, new_ty),
            4 => format!("sizeof({}) * {}", new_ty, iter[0].1),
            _ => unreachable!()
        };
        let r = format!("({}) malloc ({})", ty, expr);
        content.push_str(&make_var_assign(&var, &r));
    }
    make_fn("void", &random_ident(), &param, &content)
}

pub fn generate_b4() -> String {
    let mut rng = rand::thread_rng();
    let mut ans = String::new();
    for _ in 0..3 {
        let types = vec!["short", "unsigned short", "int", "unsigned int", "char", "unsigned char"];
        let mut vars = Vec::new();
        let mut content = String::new();
        let mut param = String::new();
        for i in 0..5 {
            let name = random_ident();
            let ty = types.choose(&mut rng).unwrap();
            if i < 2 {
                param.push_str(&make_fn_param(ty, &name));
                if i < 1 { param.push_str(", "); }
            } else {
                content.push_str(&make_var_def(ty, &name));
            }
            vars.push(name);
        }
        for _ in 0..2 {
            let iter = vars.choose_multiple(&mut rng, 2).collect::<Vec<_>>();
            let number = rand::random::<usize>();
            let condition = match rand::random::<usize>() % 9 {
                0 => format!("{} <= {}", iter[0], number),
                1 => format!("{} >= {}", iter[0], number),
                2 => format!("{} < {}", iter[0], number),
                3 => format!("{} > {}", iter[0], number),
                4 => format!("{} <= {}", number, iter[0]),
                5 => format!("{} >= {}", number, iter[0]),
                6 => format!("{} < {}", number, iter[0]),
                7 => format!("{} > {}", number, iter[0]),
                8 => format!("{} == {}", iter[0], number),
                _ => unreachable!()
            };
            content.push_str(&make_if(&condition, "return;"));
        }
        ans += &make_fn("void", &random_ident(), &param, &content);
    }
    ans
}

pub fn generate_b5() -> String {
    let mut rng = rand::thread_rng();
    let mut ans = String::new();
    for _ in 0..2 {
        let types = vec!["short*", "unsigned short*", "int*", "unsigned int*", "char*", "unsigned char*"];
        let mut vars = Vec::new();
        let mut content = String::new();
        let mut param = String::new();
        for i in 0..10 {
            let name = random_ident();
            let ty = *types.choose(&mut rng).unwrap();
            if i < 2 {
                param.push_str(&make_fn_param(ty, &name));
                if i < 1 { param.push_str(", "); }
            } else {
                content.push_str(&make_var_def(ty, &name));
            }
            vars.push((ty, name));
        }
        for _ in 0..20 {
            let iter = vars.choose_multiple(&mut rng, 2).collect::<Vec<_>>();
            let number = rand::random::<usize>() % 10;
            let expr = match rand::random::<usize>() % 7 {
                0 => format!("*{} = NULL", iter[0].1),
                1 => format!("*{} = {}", iter[0].1, iter[1].1),
                2 => format!("*{} = *{}", iter[0].1, iter[1].1),
                3 => format!("{} = *{}", iter[0].1, iter[1].1),
                4 => format!("{} = {}", iter[0].1, iter[1].1),
                5 => format!("*{} = {}", iter[0].1, number),
                6 => format!("{} = {}", iter[0].1, number),
                _ => unreachable!()
            };
            content.push_str(&expr);
            content.push_str(";\n");
        }
        ans += &make_fn("void", &random_ident(), &param, &content);
    }
    ans
}

pub fn generate_mixed() -> String {
    let mut ans = String::new();
    for _ in 0..3 {
        match rand::random::<usize>() % 4 {
            0 => ans += &generate_b2(),
            1 => ans += &generate_b3(),
            2 => ans += &generate_b4(),
            3 => ans += &generate_b5(),
            _ => unreachable!()
        }
    }
    ans
}

fn random_ident() -> String {
    let mut rng = rand::thread_rng();
    let mut ans = String::new();
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_0123456789".chars().collect::<Vec<char>>();
    let chars_begin = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_".chars().collect::<Vec<char>>();
    ans.push(*chars_begin.choose(&mut rng).unwrap()); // non-empty (unwrap)
    for _ in 0..9 {
        ans.push(*chars.choose(&mut rng).unwrap());
    }
    ans
}

fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let mut ans = String::new();
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_0123456789".chars().collect::<Vec<char>>();
    for _ in 0..rand::random::<usize>() % 30 {
        ans.push(*chars.choose(&mut rng).unwrap());
    }
    ans
}
