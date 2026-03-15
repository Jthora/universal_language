declare module "ul-wasm" {
  export function parse(input: string): any;
  export function validate(gir_json: string): any;
  export function render(gir_json: string): string;
  export function deparse(gir_json: string): string;

  type InitInput =
    | RequestInfo
    | URL
    | Response
    | BufferSource
    | WebAssembly.Module;

  export default function init(
    module_or_path?: InitInput | Promise<InitInput>
  ): Promise<any>;
}
