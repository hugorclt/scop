#include <iostream>
#include "Vec3.hpp"

Vec3::Vec3() {
    _x = 0;
    _y = 0;
    _z = 0;
    _w = 0;
}

Vec3::Vec3(double x, double y, double z) {
    _x = x;
    _y = y;
    _z = z;
    _w = 0;
}

Vec3::Vec3(double x, double y, double z, double w) {
    _x = x;
    _y = y;
    _z = z;
    _w = w;
}

Vec3::Vec3(const double &oth) {
    *this = oth;
}

Vec3::~Vec3() {

}

void Vec3::print(void) {
    std::cout << "x: " << _x << "\n"
    << "y: " << _y << "\n"
    << "z: " << _z << "\n"
    << "w: " << _w << std::endl;
}