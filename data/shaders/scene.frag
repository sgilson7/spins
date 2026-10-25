#version 300 es
precision highp float;
// Passes (the core says which): solid, lit softly (round 2: light that
// wraps past the terminator, a gentle ramp, a sky fill above and a warm
// bounce below); ink, the outline; tint, see-through shapes (rings, spell
// areas, bars). Styles: the glow style is drawn unlit. No number lives
// here; thresholds and colours are uniforms from the core.
in vec3 vWorld;
in vec3 vNormal;
in vec4 vColor;
uniform vec3 uEye;
uniform vec3 uLightDir;
uniform vec3 uSky;
uniform vec3 uFog;
uniform vec2 uFogRange;
uniform vec3 uLight;
uniform vec3 uInk;
uniform vec4 uBands;
uniform vec3 uAmbient;
uniform vec3 uGround;
uniform float uPass;
uniform vec4 uPasses;
// the glow style's number, and a half for comparing
uniform vec2 uGlow;
out vec4 outColor;

void main() {
  float d = length(vWorld - uEye);
  float f = clamp((d - uFogRange.x) / (uFogRange.y - uFogRange.x), 0.0, 1.0);
  if (abs(uPass - uPasses.z) < uPasses.w) {
    outColor = vec4(mix(vColor.rgb, uFog, f), vColor.a);
    return;
  }
  if (abs(uPass - uPasses.y) < uPasses.w) {
    outColor = vec4(mix(uInk, uFog, f), 1.0);
    return;
  }
  vec3 base = vColor.rgb;
  if (abs(vColor.a - uGlow.x) < uGlow.y) {
    outColor = vec4(mix(base, uFog, f), 1.0);
    return;
  }
  vec3 n = normalize(vNormal);
  // uBands: wrap, ramp start, ramp end, how much of the ramp; uGlow.y is a half
  float wrapped = clamp((dot(n, uLightDir) + uBands.x) / (1.0 + uBands.x), 0.0, 1.0);
  float key = mix(wrapped, smoothstep(uBands.y, uBands.z, wrapped), uBands.w);
  vec3 fill = mix(uGround, uAmbient, n.y * uGlow.y + uGlow.y);
  vec3 lit = base * (fill + uLight * key);
  outColor = vec4(mix(lit, uFog, f), 1.0);
}
