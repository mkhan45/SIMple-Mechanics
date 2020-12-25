accel_fac = 0.05

frame = 0
function update()
    -- if frame % 100 == 0 then
    --     print("FPS: ", FPS)
    -- end
    -- frame = frame + 1
end

function custom_update(obj)
    local target_x, target_y = SCREEN_X / 2, SCREEN_Y / 2
    obj.x_vel = obj.x_vel + (target_x - obj.x) * accel_fac
    obj.y_vel = obj.y_vel + (target_x - obj.y) * accel_fac
    obj.color.r = (obj.color.r + 1)
    print(obj.color.r)
    return obj
end

add_shape({shape = "circle", x = SCREEN_X / 3, y = SCREEN_Y / 3, r = 1, y_vel = 10, mass = 1, update_function="custom_update"})

GRAVITY = 0
