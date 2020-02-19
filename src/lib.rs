struct Boomer {}

fn ok_boomer() -> Result<Boomer, ()> {
    Ok(Boomer {})
}

fn all_star(i_aint_the_sharpest_tool_in_the_shed: Option<i32>) {
    let the_shape_of_an_l_on_her_forehead = 0..5;
    let live_for_fun = true;
    let fed_to_the_rules = true;
    let hit_the_ground_running = true;
    let the_years_start_coming = true;
    let they_dont_stop_coming = true;
    let gold = true;
    let all_that_glitters = true;

    if let Some(body_once_told_me_the_world_was_gonna_roll_me) =
        i_aint_the_sharpest_tool_in_the_shed
    {
        'the_mold: for she_was_looking_kinda_dumb_with_her_finger_and_her_thumb in
            the_shape_of_an_l_on_her_forehead
        {
            if the_years_start_coming && they_dont_stop_coming
                || fed_to_the_rules && hit_the_ground_running
            {
                let didnt_make_sense = !live_for_fun;
                if (all_that_glitters == gold) {
                    break 'the_mold;
                }
            }
        }
    }
}
