local function integrate(x, y, v_x, v_y, dt)
    return {
        new_x = x + v_x * dt,
        new_y = y + v_y * dt,
    }
end

X_VEL = 1
Y_VEL = -1

function update_fn(obj)
    local old_x, old_y = obj.x, obj.y
    local data = integrate(old_x, old_y, X_VEL, Y_VEL, DT / 100)
    obj.x, obj.y = data.new_x, data.new_y
    return obj
end

add_shape {
    shape="circle",
    x=SCREEN_X/2,
    y=SCREEN_Y/2,
    r=1,
    mass=1,
    update_function="update_fn"
}

GRAVITY = 0
