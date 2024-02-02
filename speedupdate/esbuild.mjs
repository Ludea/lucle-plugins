import esbuild from "esbuild";

await esbuild.build({
  entryPoints: ["src/Index.tsx"],
  bundle: true,
  sourcemap: true,
  format: 'cjs',
  target: ['es2020'],
  outdir: 'dist',
  //outfile: "dist/Index.js",
  minify: true,
  loader: {
    ".ts": "ts",
  },
});
