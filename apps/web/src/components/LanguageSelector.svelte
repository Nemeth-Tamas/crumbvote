<script lang="ts">
    import { locale, setLocale, translate, type Locale } from "../lib/i18n";

    function handleChange(changeEvent: Event) {
        const select = changeEvent.currentTarget as HTMLSelectElement;

        setLocale(select.value as Locale);
    }
</script>

<select
    data-testid="language-selector"
    aria-label={translate($locale, "language")}
    value={$locale}
    onchange={handleChange}
    class="rounded-xl border border-white/10 bg-slate-950/70 px-3 py-2 text-xs font-medium text-slate-300 outline-none transition hover:border-white/20 hover:text-white focus:border-violet-400/40"
>
    <option value="hu"> Magyar </option>

    <option value="en"> English </option>

    <option value="de"> Deutsch </option>
</select>
