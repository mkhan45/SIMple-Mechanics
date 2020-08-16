rad = 1.75

function make_row(data)
    add_shapes(
        --mA
        {shape = "circle", x = rad + 1, y = data["y_pos"], r = rad, 
        mass = data["mA"], elasticity = data["elA"], x_vel = data["vA"], name = "Mass A"},
        --mB
        {shape = "circle", x = SCREEN_X / 2, y = data["y_pos"], r = rad, 
        mass = data["mB"], elasticity = data["elB"], x_vel = data["vB"],
        color = {r = 0, g = 255, b = 0}, name = "Mass B"}
    )
end

-- purely elastic, mA == mB
make_row({elA = 1, elB = 1, mA = 1, mB = 1, vA = 5, vB = 0, y_pos = rad * 2 + 1})

-- purely elastic, mA =/= mB
make_row({elA = 1, elB = 1, mA = 1, mB = 5, vA = 5, vB = 0, y_pos = (rad * 2 + 1) * 2})

-- completely inelastic, mA == mB
make_row({elA = 0, elB = 0, mA = 1, mB = 1, vA = 5, vB = 0, y_pos = (rad * 2 + 1) * 3})

-- completely inelastic, mA =/= mB
make_row({elA = 0, elB = 0, mA = 1, mB = 5, vA = 5, vB = 0, y_pos = (rad * 2 + 1) * 4})

PAUSED = true
