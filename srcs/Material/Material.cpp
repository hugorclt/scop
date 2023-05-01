#include "Material.hpp"
#include <cstring> 

Material::Material(std::string path) {
	std::cout << path << std::endl;
	_file.open(path); //protect
	if (!_file.good())
		std::cout << "file material error zinc" << std::endl;
	_name = "";
	_textureMap = "";
	_ambiantColor = nullptr;
	_diffuseColor = nullptr;
	_specularColor = nullptr;
	_illumination = 0;
	_shininess = 0;
	_transparency = 0;
}

Material::~Material(void) {
	_file.close();
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

std::string Material::nextLine(void) {
	std::string str;

	std::getline(_file, str);
	return (str);
}

void Material::print() {
	std::cout << "MATERIAL:" << std::endl;
	std::cout << "name: " << _name << std::endl;
	std::cout << "textureMap: " << _textureMap << std::endl;
	std::cout << "refraction: " <<_refraction << std::endl;
	std::cout << "illumination: " << _illumination << std::endl;
	std::cout << "shininess: " << _shininess << std::endl;
	_ambiantColor->print();
	_diffuseColor->print();
	_specularColor->print();
	std::cout << "transparency: " << _transparency << std::endl << std::endl;
}

/* ------------------------------ getter/setter ----------------------------- */

void	Material::setName(std::string name) {
	_name = name;
}

void	Material::setTextureMap(std::string textureMap) {
	_textureMap = textureMap;
}

void	Material::setAmbiant(Vec3 *ambiant) {
	_ambiantColor = ambiant;
}

void	Material::setDiffuse(Vec3 *diffuse) {
	_diffuseColor = diffuse;
}

void	Material::setSpecular(Vec3 *specular) {
	_specularColor = specular;
}

void	Material::setRefraction(double refraction) {
	_refraction = refraction;
}

void	Material::setIllumination(int illumination) {
	_illumination = illumination;
}

void	Material::setShininess(double shininess) {
	_shininess = shininess;
}

void 	Material::setTransparency(double transparency) {
	_transparency = transparency;
}

std::string	Material::getName(void) {
	return _name;
}
	