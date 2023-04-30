#include "Face.hpp"

Face::Face() {
    _vertex = nullptr;
    _texture = nullptr;
    _normal = nullptr;
    _material = nullptr;
}

Face::Face(Vec3 *vertex, Vec3 *texture, Vec3 *normal, Material *material) {
    _vertex = vertex;
    _texture = texture;
    _normal = normal;
    _material = material;
}

Face::Face(const Face &oth) {
    *this = oth;
}

Face::~Face() {
    if (_vertex) {        
        delete _vertex;
        _vertex = nullptr;
    }
    if (_texture) {
        delete _texture;
        _texture = nullptr;
    }
    if (_normal) {
        delete _normal;
        _normal = nullptr;
    }
    if (_material) {
        delete _material;
        _material = nullptr;
    }
}