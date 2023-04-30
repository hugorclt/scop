#include "Material.hpp"

Material::Material(void) {
	_path = "";
	_name = "";
	_textureMap = "";
	_ambiantColor = nullptr;
	_diffuseColor = nullptr;
	_specularColor = nullptr;
	_illumination = 0;
	_shininess = 0;
	_transparency = 0;
}

Material::Material(int path, int name, int textureMap, Vec3 *ambiant, Vec3 *diffuse, Vec3 *specular, int illum, int shiny, int transparency) {
	_path = path;
	_name = name;
	_textureMap = textureMap;
	_ambiantColor = ambiant;
	_diffuseColor = diffuse;
	_specularColor = specular;
	_illumination = illum;
	_shininess = shiny;
	_transparency = transparency;
}

Material::Material(const Material &oth) {
	*this = oth;
}

Material::~Material(void) {
	if (_ambiantColor) {
		delete _ambiantColor;
		_ambiantColor = nullptr;
	}
	if (_diffuseColor) {
		delete _diffuseColor;
		_diffuseColor = nullptr;
	}
	if (_specularColor) {
		delete _specularColor;
		_specularColor = nullptr;
	}

}
