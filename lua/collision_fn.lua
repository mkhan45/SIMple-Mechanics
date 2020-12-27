function collide_fn(c1, c2)
    c1.x_vel, c2.x_vel = c2.x_vel, c1.x_vel
end

GRAVITY = 0

Vector = {x = 0, y = 0}
Vector.__index = Vector

function Vector:create(x, y)
    local v = {}
    setmetatable(v, Vector)
    v.x, v.y = x, y
    return v
end

function Vector:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y)
end

function Vector:dist(v)
    local radius = Vector:create(self.x - v.x, self.y - v.y)
    return radius:magnitude()
end

function Vector:dot(v)
    return self.x * v.x + self.y * v.y
end

function Vector:angle(v)
    local mag1, mag2 = self:magnitude(), v:magnitude()
    return self:dot(v) / (mag1 * mag2)
end

local function circle_collide(p1, r1, p2, r2)
    return p1:dist(p2) <= r1 + r2
end

local circ1 = {shape="circle", x = SCREEN_X / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = 2,
               update_function="circle_update", collision = "false", name="left"}

local circ2 = {shape="circle", x = SCREEN_X * 5 / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = -2.5,
               update_function="circle_update", collision = "false", name="right"}

function circle_update(obj)
    if (obj.name == "left") then
        obj.x_vel = circ1.x_vel
        circ1 = obj
    elseif (obj.name == "right") then
        obj.x_vel = circ2.x_vel
        circ2 = obj
    end

    return obj
end

function update()
    local p1 = Vector:create(circ1.x, circ1.y)
    local p2 = Vector:create(circ2.x, circ2.y)

    if (circle_collide(p1, 1, p2, 1)) then
        local relative_vel = Vector:create(circ1.x_vel - circ2.x_vel, circ1.y_vel - circ2.x_vel)
        local radius = Vector:create(p1.x - p2.x, p1.y - p2.y)
        local angle = relative_vel:angle(radius)
        if (angle < 0) then
            collide_fn(circ1, circ2)
        end
    end
end

add_shapes(circ1, circ2)
