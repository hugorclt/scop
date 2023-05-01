#include "Scene.hpp"

void	Scene::setName(std::string name) {
	_name = name;
}

void	Scene::addVertex(Vec3 *vertex) {
	_vertex.push_back(vertex);
}

void	Scene::addTexture(Vec3 *texture) {
	_textures.push_back(texture);
}

void	Scene::addNormal(Vec3 *normal) {
	_normals.push_back(normal);
}

void	Scene::addMaterial(std::string name, Material *material) {
	_materials.insert({name , material});
}

void	Scene::addFace(Face *face) {
	_faces.push_back(face);
}

void	Scene::shading(bool shading) {
	_shading = shading;
}