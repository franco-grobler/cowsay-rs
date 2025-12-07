/* tslint:disable */
/* eslint-disable */
/**
 * Return a list of all selectable cows
 */
export function listCows(): string[];
/**
 * Main entry point for the WebAssembly module.
 *
 * This function is automatically called when the WASM module is instantiated.
 * It performs necessary setup, including:
 * - Calling `before_main` for global constructor initialization.
 * - Setting up a panic hook for better error messages in the browser console
 *   when the `console_error_panic_hook` feature is enabled.
 * - Initialising the `console_log` logger to output messages to the browser's
 *   developer console.
 *
 * # Panics
 * WASM could not be initialised if the logger fails to initialize.
 */
export function run(): void;
/**
 * Defines the configuration options for a `cowsay` message in a WebAssembly context.
 *
 * This struct holds all the customizable parameters for generating a cowsay
 * message, such as the cow's appearance (e.g., `borg`, `dead`), the cowfile
 * to use, eye and tongue strings, and text wrapping settings.
 *
 * Instances of `Options` are created using the `new` constructor or
 * `default_options` for a default configuration.
 */
export class Options {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Creates a new `Options` instance with default values.
   *
   * This provides a convenient way to get a default set of options, which
   * can then be used to generate a standard cowsay message.
   *
   * # Returns
   *
   * An `Options` struct with default settings.
   *
   * # Examples
   *
   * ```javascript
   * import { Options } from 'cowsay-wasm';
   *
   * const defaultOptions = Options.defaultOptions();
   * ```
   */
  static defaultOptions(): Options;
  /**
   * Creates a new `Options` instance from a JavaScript object.
   *
   * This constructor accepts a `JsValue` representing a JavaScript object
   * with configuration properties. Missing properties will be set to their
   * default values.
   *
   * # Arguments
   *
   * * `options` - A `JsValue` containing the configuration options.
   *
   * # Returns
   *
   * A `Result` which is:
   * - `Ok(Options)` if the `JsValue` is successfully parsed.
   * - `Err(Error)` if the `JsValue` cannot be deserialized into `OptionsConstructor`.
   *
   * # Examples
   *
   * ```javascript
   * import { Options } from 'cowsay-wasm';
   *
   * const options = new Options({
   *   file: 'tux',
   *   eyes: '^^',
   * });
   * ```
   */
  constructor(options: any);
  /**
   * Generates the cowsay message based on the configured options.
   *
   * This method takes the current `Options` and a message string, then
   * invokes the core `cowsay` logic to produce the final ASCII art.
   *
   * # Arguments
   *
   * * `message` - The text message for the cow to say.
   *
   * # Returns
   *
   * A `Result` which is:
   * - `Ok(String)` containing the complete cowsay message.
   * - `Err(Error)` if there is an issue with parsing the options or
   *   generating the message (e.g., cowfile not found).
   *
   * # Examples
   *
   * ```javascript
   * import { Options } from 'cowsay-wasm';
   *
   * const options = Options.default_options();
   * const cowMessage = options.say('Moo!');
   * console.log(cowMessage);
   * ```
   */
  say(message: string): string;
}
/**
 * A helper struct for deserializing `Options` from a JavaScript object.
 *
 * This struct is used internally by the `Options::new` constructor to
 * deserialize a `JsValue` into a structured format. All fields are optional
 * to allow for partial configuration from the JavaScript side.
 */
export class OptionsConstructor {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_options_free: (a: number, b: number) => void;
  readonly __wbg_optionsconstructor_free: (a: number, b: number) => void;
  readonly options_defaultOptions: () => number;
  readonly options_new: (a: any) => [number, number, number];
  readonly options_say: (a: number, b: number, c: number) => [number, number, number, number];
  readonly listCows: () => [number, number];
  readonly run: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
