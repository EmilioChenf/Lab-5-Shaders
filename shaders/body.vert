#version 120
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
uniform float Spin;
uniform vec3 Center;
uniform float Radius;
uniform float Tilt;
varying vec3 local;
varying vec3 normalWorld;
varying vec3 world;
void main() {
    local = position;
    float c=cos(Spin), s=sin(Spin);
    mat3 rotation=mat3(c,0.,-s, 0.,1.,0., s,0.,c);
    vec3 p=rotation*position;
    mat3 tilt=mat3(1.,0.,0., 0.,cos(Tilt),sin(Tilt), 0.,-sin(Tilt),cos(Tilt));
    vec4 w=Model*vec4(Center+tilt*p*Radius,1.);
    world=w.xyz;
    normalWorld=normalize(mat3(Model)*tilt*rotation*normalize(position));
    gl_Position=Projection*w;
}
