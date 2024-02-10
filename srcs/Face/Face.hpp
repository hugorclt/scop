#pragma once
#include "Vec3.hpp"
#include "Material.hpp"


//TRIANGLE
class Face {
    private:
        Vec3        *_vertex;
        Vec3        *_texture;
        Vec3        *_normal;
        Material    *_material;

    public:
        Face();
        Face(Vec3 *vertex, Vec3 *texture, Vec3 *normal, Material *material);
        Face(const Face &oth);

        ~Face();
};