-- this function is called on the circles when they collide
function collide_fn(c1, c2)
    -- 1D elastic collision
    local m1 = c1.mass
    local m2 = c2.mass

    local v1 = c1.x_vel
    local v2 = c2.x_vel

    return {
        v1_f = (m1 - m2) / (m1 + m2) * v1 + 2 * m2 / (m1 + m2) * v2,
        v2_f = (m2 - m1) / (m1 + m2) * v2 + 2 * m1 / (m1 + m2) * v1
    }
end

GRAVITY = 0

-- defining a Vector class
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

function Vector:cos_angle(v)
    local mag1, mag2 = self:magnitude(), v:magnitude()
    return self:dot(v) / (mag1 * mag2)
end

function Vector:__add(v)
    return Vector:create(self.x + v.x, self.y + v.y)
end

function Vector:__sub(v)
    return Vector:create(self.x - v.x, self.y - v.y)
end

function Vector:__mul(f)
    if (type(self) == "table") then
        return Vector:create(self.x * f, self.y * f)
    else
        return Vector:create(f.x * self, f.y * self)
    end
end

function Vector:__div(f)
    return self * (1 / f)
end
----

-- checks if the circles are colliding
local function circle_collide(p1, r1, p2, r2)
    return p1:dist(p2) <= r1 + r2
end

-- defining the circle variables which will be used to reflect the
-- simulation data
local circ1 = {shape="circle", x = SCREEN_X / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = 2,
update_function="circle_update", collision = "false", name="circ1"}

local circ2 = {shape="circle", x = SCREEN_X * 5 / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = -2.5,
update_function="circle_update", collision = "false", name="circ2"}

-- if the Lua circles are changed, update the physics sim to reflect them
-- otherwise, update the Lua circles to reflect the physics sim
function circle_update(obj)
    if (obj.name == "circ1") then
        if (circ1.changed) then
            obj = circ1
            circ1.changed = false
        end
        circ1 = obj
    elseif (obj.name == "circ2") then
        if (circ2.changed) then
            obj = circ2
            circ2.changed = false
        end
        circ2 = obj
    end

    obj.y_vel = 0

    return obj
end

-- every frame, check if the circles are colliding and call
-- the collide_fn if they collide
function update()
    local p1 = Vector:create(circ1.x, circ1.y)
    local p2 = Vector:create(circ2.x, circ2.y)

    if (circle_collide(p1, 1, p2, 1)) then
        local relative_vel = Vector:create(circ1.x_vel - circ2.x_vel, circ1.y_vel - circ2.x_vel)
        local radius = Vector:create(p1.x - p2.x, p1.y - p2.y)
        local cos_angle = relative_vel:cos_angle(radius)
        if (cos_angle < 0) then
            circ1.changed, circ2.changed = true, true
            local collision_res = collide_fn(circ1, circ2)
            circ1.x_vel = collision_res.v1_f
            circ2.x_vel = collision_res.v2_f
        end
    end
end

-- add the shapes to the scene
add_shapes(circ1, circ2)
