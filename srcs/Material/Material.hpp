#pragma once
#include <string>
#include "Vec3.hpp"

class Material {
    private:
        std::string	_path;
        std::string	_name;
		std::string	_textureMap;
        Vec3		*_ambiantColor;
		Vec3		*_diffuseColor;
		Vec3		*_specularColor;
		int			_illumination;
		double		_shininess;
		double		_transparency;

    public:
		Material(void);
		Material(int path, int name, int textureMap, Vec3 *ambiant, Vec3 *diffuse, Vec3 *specular, int illum, int shiny, int transparency);
		Material(const Material &oth);

		~Material(void);
};