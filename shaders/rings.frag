#version 100
precision highp float;
varying vec3 local;
void main() {
    float r=length(local.xz);
    // Actual geometric annulus; radial gaps and mineral bands are shader patterns.
    if(abs(r-1.68)<.045 || abs(r-1.91)<.015) discard;
    float bands=sin(r*180.)*.5+.5;
    vec3 c=mix(vec3(.25,.15,.10),vec3(.87,.72,.43),bands);
    c*=.7+.3*sin(r*47.);
    gl_FragColor=vec4(c,1.);
}
