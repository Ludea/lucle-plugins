import esbuild from "esbuild";

await esbuild.build({
  entryPoints: ["src/Index.tsx"],
  bundle: true,
  outfile: "dist/out.js",
  minify: true,
  loader: {
    ".ts": "ts",
  },
});
