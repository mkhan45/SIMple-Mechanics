function collide_fn(c1, c2)
    -- 2D elastic collision between equal mass objects

    local v1 = Vector:create(c1.x_vel, c1.y_vel)
    local v2 = Vector:create(c2.x_vel, c2.y_vel)
    local relative_vel = v1 - v2
    local inv_relative_vel = v2 - v1

    print(a.c)

    local p1 = Vector:create(c1.x, c1.y)
    local p2 = Vector:create(c2.x, c2.y)
    local radius = p1 - p2
    local inv_radius = p2 - p1
    local radius_mag = radius:magnitude()

    local v1_final = v1 - (relative_vel:dot(radius) / (radius_mag * radius_mag) * radius)
    local v2_final = v2 - (inv_relative_vel:dot(inv_radius) / (radius_mag * radius_mag) * inv_radius)

    c1.x_vel, c1.y_vel = v1_final.x, v1_final.y
    c2.x_vel, c2.y_vel = v2_final.x, v2_final.y
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

function Vector:cos_angle(v)
    local mag1, mag2 = self:magnitude(), v:magnitude()
    return self:dot(v) / (mag1 * mag2)
end

function Vector:__add(v)
    return Vector:create(self.x + v.x, self.y + v.y)
end
-- Vector.mt.__add = Vector.add

function Vector:__sub(v)
    return Vector:create(self.x - v.x, self.y - v.y)
end
-- Vector.mt.__sub = Vector.sub

function Vector:__mul(f)
    if (type(self) == "table") then
        return Vector:create(self.x * f, self.y * f)
    else
        return Vector:create(f.x * self, f.y * self)
    end
end
-- Vector.mt.__mul = Vector.mul

function Vector:__div(f)
    return self * (1 / f)
end
-- Vector.mt.__div = Vector.div

local function circle_collide(p1, r1, p2, r2)
    return p1:dist(p2) <= r1 + r2
end

local circ1 = {shape="circle", x = SCREEN_X / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = 2,
               update_function="circle_update", collision = "false", name="circ1"}

local circ2 = {shape="circle", x = SCREEN_X * 5 / 6, y = SCREEN_Y / 2, r = 1, mass = 1, x_vel = -2.5,
               update_function="circle_update", collision = "false", name="circ2"}

function circle_update(obj)
    if (obj.name == "circ1") then
        if (circ1.changed) then
            -- obj.x_vel, obj.y_vel = circ1.x_vel, circ1.y_vel
            obj = circ1
            circ1.changed = false
        end
        circ1 = obj
    elseif (obj.name == "circ2") then
        if (circ2.changed) then
            -- obj.x_vel, obj.y_vel = circ2.x_vel, circ2.y_vel
            obj = circ2
            circ2.changed = false
        end
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
        local cos_angle = relative_vel:cos_angle(radius)
        if (cos_angle < 0) then
            circ1.changed, circ2.changed = true, true
            collide_fn(circ1, circ2)
        end
    end
end

add_shapes(circ1, circ2)
