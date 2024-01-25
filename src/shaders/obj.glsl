@header use crate::system::math::math as m

@ctype mat4 m::Mat4
@ctype vec3 m::Vec3

@vs vs
in vec3 aPos;
in vec3 aNormal;

out vec3 FragPos;
out vec3 Normal;

uniform ObjVsParams {
    mat4 model;
    mat4 view;
    mat4 projection;
};

void main() {
    FragPos = vec3(model * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * aNormal;  
    
    gl_Position = projection * view * vec4(FragPos, 1.0);
}
@end

@fs fs
out vec4 FragColor;

in vec3 FragPos;  
in vec3 Normal; 


uniform ObjFsParams {
    vec3 lightPos;
    vec3 viewPos;
    vec3 lightColor;
    vec3 objectColor;
};

void main() {
    // ambient
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;
  	
    // diffuse 
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
    
    // specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);  
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;  
        
    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
@end

@program obj vs fs