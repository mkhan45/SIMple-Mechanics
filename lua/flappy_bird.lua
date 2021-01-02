-- since Lua can't access keypresses this will be AI driven

PIPE_WIDTH = 1.5
PIPE_SPEED = 15
PIPE_COLOR = {r = 0, g = 255, b = 0}

GAP_HEIGHT = 7
GAP_MIN_Y = SCREEN_Y / 4
GAP_MAX_Y = 3 / 4 * SCREEN_Y
CURRENT_GAP_Y = 0

BIRD_JUMP_ACCEL = 20
BIRD_JUMP_COOLDOWN = 10
CURRENT_JUMP_COOLDOWN = 0
BIRD_RADIUS = 0.9
BIRD_GRAVITY = 0.075
BIRD_COLOR = {r = 255, g = 255, b = 0}

local function reset_gap_y()
    CURRENT_GAP_Y = math.random(0, 100) / 100 * (GAP_MAX_Y - GAP_MIN_Y) + GAP_MIN_Y
end
reset_gap_y()

local function calculate_top_pipe_y()
    return CURRENT_GAP_Y - (GAP_HEIGHT / 2) - (SCREEN_Y / 2)
end

local function calculate_bottom_pipe_y()
    return CURRENT_GAP_Y + (GAP_HEIGHT / 2) + (SCREEN_Y / 2)
end

local function bird_jump(bird)
    bird.y_vel = bird.y_vel - BIRD_JUMP_ACCEL
    CURRENT_JUMP_COOLDOWN = BIRD_JUMP_COOLDOWN
end

local function should_jump(obj)
    local below_gap = obj.y > CURRENT_GAP_Y + GAP_HEIGHT / 2 - (BIRD_RADIUS * 2.5)
    local jump_available = CURRENT_JUMP_COOLDOWN < 0
    local not_too_fast = obj.y_vel > -BIRD_JUMP_ACCEL / 8

    return below_gap and jump_available and not_too_fast
end

function bird_update(obj)
    obj.y_vel = obj.y_vel + BIRD_GRAVITY * DT

    if should_jump(obj) then
        bird_jump(obj)
    end

    CURRENT_JUMP_COOLDOWN = CURRENT_JUMP_COOLDOWN - 1 * DT

    return obj
end

GAP_RESET = false
function pipe_update(obj)
    if obj.x + PIPE_WIDTH < 0 then
        obj.x = SCREEN_X + PIPE_WIDTH
        GAP_RESET = not GAP_RESET

        if GAP_RESET then
            reset_gap_y()
        end

        if obj.name == "top_pipe" then
            obj.y = calculate_top_pipe_y()
        else
            obj.y = calculate_bottom_pipe_y()
        end
    end
    return obj
end

add_shape{
    shape="circle",
    x = SCREEN_X / 8,
    y = SCREEN_Y / 2,
    r = BIRD_RADIUS,
    mass = 100,
    update_function = "bird_update",
    color=BIRD_COLOR
}

-- upper pipe
add_shape{
    shape="rect",
    x=SCREEN_X + PIPE_WIDTH,
    y = calculate_top_pipe_y(),
    w = PIPE_WIDTH,
    h = SCREEN_Y / 2,
    mass = 1,
    x_vel = -PIPE_SPEED,
    update_function="pipe_update",
    name="top_pipe",
    color=PIPE_COLOR
}

-- lower pipe
add_shape{
    shape="rect",
    x=SCREEN_X + PIPE_WIDTH,
    y = calculate_bottom_pipe_y(),
    w = PIPE_WIDTH,
    h = SCREEN_Y / 2,
    mass = 1,
    x_vel = -PIPE_SPEED,
    update_function="pipe_update",
    name="bottom_pipe",
    color=PIPE_COLOR
}

-- so the pipes don't fall; the bird has artificial Lua gravity
GRAVITY = 0
