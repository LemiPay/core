<script lang="ts">
    interface Props {
        id: string;
        label: string;
        placeholder?: string;
        min?: number;
        max?: number;
        step?: number | 'any';
        value: string | number; // Acepta ambos para no romper tu estado $state('')
        attempted?: boolean;
    }

    let {
        id,
        label,
        placeholder = '',
        min,
        max,
        step = 'any', // Clave para permitir decimales sin que el navegador tire error
        value = $bindable(''),
        attempted = false
    }: Props = $props();

    let touched = $state(false);

    const showFeedback = $derived(touched || attempted);

    // Convertimos el valor asegurándonos de aceptar comas y puntos
    const parsedValue = $derived(
        value === '' || value == null
            ? NaN
            : Number(String(value).replace(',', '.'))
    );

    const isEmpty = $derived(value === '' || value == null);
    const isInvalid = $derived(!isEmpty && isNaN(parsedValue));
    const underMin = $derived(min !== undefined && !isEmpty && !isInvalid && parsedValue < min);
    const overMax = $derived(max !== undefined && !isEmpty && !isInvalid && parsedValue > max);

    const isValid = $derived(!isEmpty && !isInvalid && !underMin && !overMax);

    const message = $derived(
        isEmpty
            ? `${label} es requerido`
            : isInvalid
                ? `Ingresá un número válido`
                : underMin
                    ? `El mínimo es ${min}`
                    : overMax
                        ? `El máximo es ${max}`
                        : '¡Se ve bien!'
    );
</script>

<div>
    <label for={id} class="mb-1.5 block text-sm font-medium text-black">{label}</label>

    <input
            {id}
            type="number"
            {step}
            {min}
            {max}
            bind:value
            {placeholder}
            onblur={() => (touched = true)}
            class="w-full rounded-md border px-3 py-2 text-sm text-black placeholder-gray-400 transition focus:ring-0 focus:outline-none
          {showFeedback
          ? isValid
             ? 'border-green-400 focus:border-green-500'
             : 'border-red-400 focus:border-red-500'
          : 'border-gray-200 focus:border-gray-400'}"
    />

    {#if showFeedback}
        <div
                class="mt-1.5 flex items-center text-xs
             {isValid ? 'text-green-600' : 'text-red-500'}"
        >
          <span class="flex items-center gap-1">
             {#if isValid}
                <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-3.5 w-3.5 shrink-0"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                >
                   <polyline points="20 6 9 17 4 12" />
                </svg>
             {:else}
                <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-3.5 w-3.5 shrink-0"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                >
                   <line x1="18" y1="6" x2="6" y2="18" />
                   <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
             {/if}
              {message}
          </span>
        </div>
        {:else}
        <div class="mt-1.5 h-[18px]"></div>
    {/if}
</div>