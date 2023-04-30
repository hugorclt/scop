#pragma once
#include <string>
#include <fstream>
#include <iostream>
#include <map>
#include "Vec3.hpp"
#include "Material.hpp"
#include "Face.hpp"
#include "Scene.hpp"

class Parser {
	private:
		using ParseFunc = void (Parser::*)(std::string, Scene&);
		std::ifstream	_file;

		
		void 	_parseMaterial(std::string line, Scene &scene);
		void 	_parseComment(std::string line, Scene &scene);
		void	_parseVertex(std::string line, Scene &scene);
		void	_parseTexture(std::string line, Scene &scene);
		void	_parseNormals(std::string line, Scene &scene);
		void	_parseGroup(std::string line, Scene &scene);
		void	_parseName(std::string line, Scene &scene);
		void	_useMaterial(std::string line, Scene &scene);
		void	_parseShading(std::string line, Scene &scene);
		void	_parseFace(std::string line, Scene &scene);
		void	_parseLine(std::string line, Scene &scene);

		/* ---------------------------------- utils --------------------------------- */
		std::string _nextLine(void);
		std::string _getFirstWord(std::string line);

		static const std::map<std::string, ParseFunc> _lineType;

	public:
		Parser(std::string path);
		~Parser(void);

		void	parseObj(Scene &scene);

		class FileError : public std::exception{
			public:
				virtual const char 	*what() const throw()
				{
					return ("Error while opening the file");
				}
		};

};