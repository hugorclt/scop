#pragma once
#include <vector>
#include <string>
#include "Face.hpp"


class Scene {
	private:
		std::string 			_name;
		std::vector<Vec3>		_vertex;
		std::vector<Vec3>		_textures;
		std::vector<Vec3>		_normals;
		std::vector<Material>	_materials;
		std::vector<Face>		_faces;
		bool					shading;

	public:
		void	setName(std::string name);
};