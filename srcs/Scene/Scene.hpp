#pragma once
#include <vector>
#include <string>
#include "Face.hpp"


class Scene {
	private:
		std::string 			_name;
		std::vector<Vec3*>		_vertex;
		std::vector<Vec3*>		_textures;
		std::vector<Vec3*>		_normals;
		std::vector<Material*>	_materials;
		std::vector<Face*>		_faces;
		bool					_shading;

	public:
		void	setName(std::string name);
		void	addVertex(Vec3 *vertex);
		void	addTexture(Vec3 *texture);
		void	addNormal(Vec3 *texture);
		void	addMaterial(Material *material);
		void	addFace(Face *face);
		void	shading(bool shading);
};