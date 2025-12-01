import init, { Options, listCows } from "cowsay-wasm";
import { ref } from "vue";

export interface CowsayOptions {
	borg?: boolean;
	dead?: boolean;
	greedy?: boolean;
	sleepy?: boolean;
	tired?: boolean;
	wired?: boolean;
	young?: boolean;
	file?: string;
	random?: boolean;
	eyes?: string;
	tongue?: string;
	wrap?: boolean;
	wrap_column?: number;
}

export default function useCowsay() {
	const initialised = ref<boolean>(false);

	async function initCowsay() {
		if (initialised.value === true) return;
		await init();
	}

	function getCowText(text: string, data: CowsayOptions): string {
		if (!initialised.value) {
			return "Not ready...";
		}
		const options = new Options(data);
		return options.say(text);
	}

	function getAvailableCows(): string[] {
		if (!initialised.value) {
			return [];
		}
		return listCows();
	}

	initCowsay().then(() => {
		initialised.value = true;
	});

	return { initCowsay, initialised, getAvailableCows, getCowText };
}
