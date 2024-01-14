from stl import mesh
import json

def process_stl(file_path):
    try:
        stl_mesh = mesh.Mesh.from_file(file_path)

        # calculate vertices and faces of the stl file
        num_vertices = len(stl_mesh.points)
        num_faces = len(stl_mesh.faces)

        # return as JSON string
        result = {
            "num_vertices": num_vertices,
            "num_faces": num_faces,
        }
        return json.dumps(result)
    except Exception as e:
        return json.dumps({"error": str(e)})


