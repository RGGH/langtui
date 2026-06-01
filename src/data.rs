// ── Verb data & sentences ─────────────────────────────────────────────────

pub const VERBS: &[&str] = &["irse", "quedarse", "ponerse", "llevarse", "acordarse"];
pub const TENSES: &[&str] = &["presente", "pretérito", "perfecto", "condicional"];
pub const TENSE_LABELS: &[&str] = &["Pres.", "Pret.", "Perf.", "Cond."];
pub const PERSONS: &[&str] = &["yo", "tú", "él/ella"];

pub fn conjugation(verb: &str, tense: &str, person: &str) -> &'static str {
    match (verb, tense, person) {
        // irse
        ("irse", "presente",    "yo")      => "me voy",
        ("irse", "presente",    "tú")      => "te vas",
        ("irse", "presente",    "él/ella") => "se va",
        ("irse", "pretérito",   "yo")      => "me fui",
        ("irse", "pretérito",   "tú")      => "te fuiste",
        ("irse", "pretérito",   "él/ella") => "se fue",
        ("irse", "perfecto",    "yo")      => "me he ido",
        ("irse", "perfecto",    "tú")      => "te has ido",
        ("irse", "perfecto",    "él/ella") => "se ha ido",
        ("irse", "condicional", "yo")      => "me iría",
        ("irse", "condicional", "tú")      => "te irías",
        ("irse", "condicional", "él/ella") => "se iría",
        // quedarse
        ("quedarse", "presente",    "yo")      => "me quedo",
        ("quedarse", "presente",    "tú")      => "te quedas",
        ("quedarse", "presente",    "él/ella") => "se queda",
        ("quedarse", "pretérito",   "yo")      => "me quedé",
        ("quedarse", "pretérito",   "tú")      => "te quedaste",
        ("quedarse", "pretérito",   "él/ella") => "se quedó",
        ("quedarse", "perfecto",    "yo")      => "me he quedado",
        ("quedarse", "perfecto",    "tú")      => "te has quedado",
        ("quedarse", "perfecto",    "él/ella") => "se ha quedado",
        ("quedarse", "condicional", "yo")      => "me quedaría",
        ("quedarse", "condicional", "tú")      => "te quedarías",
        ("quedarse", "condicional", "él/ella") => "se quedaría",
        // ponerse
        ("ponerse", "presente",    "yo")      => "me pongo",
        ("ponerse", "presente",    "tú")      => "te pones",
        ("ponerse", "presente",    "él/ella") => "se pone",
        ("ponerse", "pretérito",   "yo")      => "me puse",
        ("ponerse", "pretérito",   "tú")      => "te pusiste",
        ("ponerse", "pretérito",   "él/ella") => "se puso",
        ("ponerse", "perfecto",    "yo")      => "me he puesto",
        ("ponerse", "perfecto",    "tú")      => "te has puesto",
        ("ponerse", "perfecto",    "él/ella") => "se ha puesto",
        ("ponerse", "condicional", "yo")      => "me pondría",
        ("ponerse", "condicional", "tú")      => "te pondrías",
        ("ponerse", "condicional", "él/ella") => "se pondría",
        // llevarse
        ("llevarse", "presente",    "yo")      => "me llevo",
        ("llevarse", "presente",    "tú")      => "te llevas",
        ("llevarse", "presente",    "él/ella") => "se lleva",
        ("llevarse", "pretérito",   "yo")      => "me llevé",
        ("llevarse", "pretérito",   "tú")      => "te llevaste",
        ("llevarse", "pretérito",   "él/ella") => "se llevó",
        ("llevarse", "perfecto",    "yo")      => "me he llevado",
        ("llevarse", "perfecto",    "tú")      => "te has llevado",
        ("llevarse", "perfecto",    "él/ella") => "se ha llevado",
        ("llevarse", "condicional", "yo")      => "me llevaría",
        ("llevarse", "condicional", "tú")      => "te llevarías",
        ("llevarse", "condicional", "él/ella") => "se llevaría",
        // acordarse
        ("acordarse", "presente",    "yo")      => "me acuerdo",
        ("acordarse", "presente",    "tú")      => "te acuerdas",
        ("acordarse", "presente",    "él/ella") => "se acuerda",
        ("acordarse", "pretérito",   "yo")      => "me acordé",
        ("acordarse", "pretérito",   "tú")      => "te acordaste",
        ("acordarse", "pretérito",   "él/ella") => "se acordó",
        ("acordarse", "perfecto",    "yo")      => "me he acordado",
        ("acordarse", "perfecto",    "tú")      => "te has acordado",
        ("acordarse", "perfecto",    "él/ella") => "se ha acordado",
        ("acordarse", "condicional", "yo")      => "me acordaría",
        ("acordarse", "condicional", "tú")      => "te acordarías",
        ("acordarse", "condicional", "él/ella") => "se acordaría",
        _ => panic!("Unknown combo: {verb} / {tense} / {person}"),
    }
}

/// Returns (before_blank, after_blank) pairs for fill-in-the-blank sentences.
pub fn sentences(verb: &str, tense: &str, person: &str) -> &'static [(&'static str, &'static str)] {
    match (verb, tense, person) {
        // ── irse ─────────────────────────────────────────────────────────────
        ("irse","presente","yo") => &[
            ("Cuando termina la fiesta, ___"," sin despedirme de nadie."),
            ("Siempre que llueve, ___"," antes de que empiece el tráfico."),
            ("No aguanto más — ___"," ahora mismo."),
        ],
        ("irse","presente","tú") => &[
            ("¿Por qué ___"," tan pronto? La cena acaba de empezar."),
            ("Si ___ ahora,"," perderás el autobús de las ocho."),
            ("Cada vez que discutimos, ___"," sin decir nada."),
        ],
        ("irse","presente","él/ella") => &[
            ("Mi hermana ___"," al trabajo antes del amanecer."),
            ("El tren ___"," en diez minutos — date prisa."),
            ("Cuando termina su turno, ___"," sin mirar atrás."),
        ],
        ("irse","pretérito","yo") => &[
            ("Anoche ___ de la fiesta"," antes de medianoche — estaba agotado."),
            ("___ del trabajo"," dos horas antes de lo normal."),
            ("Sin decir una palabra, ___"," y cerré la puerta."),
        ],
        ("irse","pretérito","tú") => &[
            ("¿A qué hora ___"," del concierto anoche?"),
            ("___ sin el paraguas"," y te mojaste hasta los huesos."),
            ("Te busqué en la sala pero ya ___"," cuando llegué."),
        ],
        ("irse","pretérito","él/ella") => &[
            ("Mi vecino ___"," de vacaciones y olvidó cerrar el gas."),
            ("Ella ___ llorando"," cuando oyó la noticia."),
            ("El último tren ___"," hace media hora — tuvimos que coger un taxi."),
        ],
        ("irse","perfecto","yo") => &[
            ("Ya ___ tres veces"," este mes y el jefe empieza a notarlo."),
            ("___ antes de que llegara la tormenta,"," así que llegué seco a casa."),
            ("No ___ de su lado"," ni un momento en toda la noche."),
        ],
        ("irse","perfecto","tú") => &[
            ("¿Ya ___? ¡Si acabas de llegar!",""),
            ("___ sin recoger tus cosas"," — te las guardo yo."),
            ("Me han dicho que ___"," a vivir a otro país. ¿Es verdad?"),
        ],
        ("irse","perfecto","él/ella") => &[
            ("El fontanero ___"," pero el grifo sigue goteando."),
            ("Luisa ___ de viaje"," y no volverá hasta el lunes."),
            ("El dolor de cabeza ___"," después de tomar la pastilla."),
        ],
        ("irse","condicional","yo") => &[
            ("___ contigo al cine,"," pero tengo que terminar este informe."),
            ("Si pudiera, ___"," a vivir al campo mañana mismo."),
            ("___ de vacaciones en verano"," si tuviera más dinero."),
        ],
        ("irse","condicional","tú") => &[
            ("¿___ sin despedirte?"," Eso no estaría bien."),
            ("Si te trataran así, ___"," sin dudarlo un segundo."),
            ("Creo que ___"," antes de la reunión si fueras listo."),
        ],
        ("irse","condicional","él/ella") => &[
            ("Mi madre dijo que ___"," si el tiempo no mejoraba."),
            ("Él ___ antes si pudiera,"," pero tiene guardia hasta las diez."),
            ("La reunión ___ sola"," si nadie la moderara."),
        ],
        // ── quedarse ─────────────────────────────────────────────────────────
        ("quedarse","presente","yo") => &[
            ("Esta noche ___"," en casa porque hay partido."),
            ("Cuando hay tormenta, ___"," bajo el porche hasta que para."),
            ("___ aquí trabajando"," mientras vosotros os divertís."),
        ],
        ("quedarse","presente","tú") => &[
            ("¿___ a cenar?"," Hay sobras del cocido."),
            ("Siempre ___ con las ganas"," de decirle lo que piensas."),
            ("Si ___ más tiempo,"," podrías verla cuando llegue."),
        ],
        ("quedarse","presente","él/ella") => &[
            ("Mi abuelo ___"," en el pueblo aunque todos se han ido a la ciudad."),
            ("El perro siempre ___"," mirando la puerta cuando salgo."),
            ("Ella ___"," sin palabras cada vez que lo ve."),
        ],
        ("quedarse","pretérito","yo") => &[
            ("___ estudiando en la biblioteca"," hasta que la cerraron."),
            ("Anoche ___ dormido en el sofá"," viendo la segunda parte del partido."),
            ("___ sin habla"," cuando me dijeron lo del ascenso."),
        ],
        ("quedarse","pretérito","tú") => &[
            ("¿___ en la oficina"," hasta tarde otra vez?"),
            ("___ sin batería en el móvil"," justo cuando más lo necesitabas."),
            ("Te ___ mirando el mar"," durante horas sin decir nada."),
        ],
        ("quedarse","pretérito","él/ella") => &[
            ("El bebé ___"," dormido en el coche."),
            ("Ella ___ en casa"," mientras los demás fueron a la playa."),
            ("Mi jefe ___"," boquiabierto cuando vio los resultados."),
        ],
        ("quedarse","perfecto","yo") => &[
            ("___ sin dinero"," a mitad de mes otra vez."),
            ("Esta semana ___ dos noches extra"," para terminar el proyecto."),
            ("___ con las ganas de preguntárselo,"," no me atreví."),
        ],
        ("quedarse","perfecto","tú") => &[
            ("¿___ a dormir en casa de Ana?"," Tu madre estaba preocupada."),
            ("___ sin cenar"," por llegar tarde. ¿Quieres que te caliente algo?"),
            ("Me han dicho que ___"," con el puesto. ¡Enhorabuena!"),
        ],
        ("quedarse","perfecto","él/ella") => &[
            ("El coche ___"," sin gasolina en la autopista."),
            ("Ella ___"," con la mitad de las entradas — nadie las quería."),
            ("Mi hermano ___"," en paro desde enero."),
        ],
        ("quedarse","condicional","yo") => &[
            ("___ contigo más tiempo,"," pero tengo el último tren."),
            ("Si hubiera buena película, ___"," en casa sin dudarlo."),
            ("___ a vivir aquí"," si el alquiler no fuera tan caro."),
        ],
        ("quedarse","condicional","tú") => &[
            ("¿___ a cuidar al gato"," si me fuera de viaje?"),
            ("___ más tranquilo"," si supieras la verdad."),
            ("En tu lugar, ___"," callado y dejarías pasar la tormenta."),
        ],
        ("quedarse","condicional","él/ella") => &[
            ("Mi padre dijo que ___"," en casa si llovía."),
            ("Ella ___"," sin trabajo si cerrara esa empresa."),
            ("El niño ___"," jugando toda la noche si le dejaran."),
        ],
        // ── ponerse ──────────────────────────────────────────────────────────
        ("ponerse","presente","yo") => &[
            ("Cuando veo una araña, ___"," a temblar como un flan."),
            ("Siempre ___"," nervioso antes de una presentación importante."),
            ("___ el abrigo"," cuando la temperatura baja de diez grados."),
        ],
        ("ponerse","presente","tú") => &[
            ("¿Por qué ___"," tan rojo cuando te hablo?"),
            ("Cada vez que lo criticas, ___"," a la defensiva."),
            ("___ los guantes"," antes de sacar al perro — está helando."),
        ],
        ("ponerse","presente","él/ella") => &[
            ("Mi hermana ___"," a llorar con cualquier película."),
            ("Él ___"," furioso si alguien toca sus cosas."),
            ("La niña ___"," contenta en cuanto ve a su abuela."),
        ],
        ("ponerse","pretérito","yo") => &[
            ("___ a llorar"," cuando vi que habían suspendido el concierto."),
            ("___ el impermeable"," y salí a buscar el paraguas que olvidé."),
            ("___ enfermo"," justo el día de mis vacaciones."),
        ],
        ("ponerse","pretérito","tú") => &[
            ("¿___ el chubasquero"," antes de salir? Llegaste empapado."),
            ("___ pálido"," cuando te dije que había un examen sorpresa."),
            ("___ a estudiar demasiado tarde,"," por eso no terminaste."),
        ],
        ("ponerse","pretérito","él/ella") => &[
            ("Mi madre ___"," muy contenta cuando le dimos la noticia."),
            ("Él ___ a gritar"," en mitad del restaurante — fue muy violento."),
            ("La actriz ___"," el vestido de noche y salió al escenario."),
        ],
        ("ponerse","perfecto","yo") => &[
            ("___ a estudiar chino"," y ya sé pedir el menú."),
            ("___ las zapatillas nuevas"," y ya me han salido ampollas."),
            ("___ muy nervioso"," con todo lo de la mudanza."),
        ],
        ("ponerse","perfecto","tú") => &[
            ("¿___ ya el protector solar?"," El sol está fuerte hoy."),
            ("___ a aprender a cocinar"," — los resultados son sorprendentes."),
            ("Te ___"," muy serio de repente. ¿Pasa algo?"),
        ],
        ("ponerse","perfecto","él/ella") => &[
            ("Mi padre ___"," enfermo y no vendrá a cenar."),
            ("Ella ___"," a escribir la novela que lleva años planeando."),
            ("El cielo ___"," gris de repente — creo que va a llover."),
        ],
        ("ponerse","condicional","yo") => &[
            ("___ a correr"," si supiera que es bueno para mis rodillas."),
            ("___ ese sombrero,"," pero me da vergüenza en el metro."),
            ("___ más nervioso todavía"," si tuviera que hablar en público."),
        ],
        ("ponerse","condicional","tú") => &[
            ("¿___ ese traje para la entrevista?"," Creo que quedaría bien."),
            ("___ a dieta"," si el médico te lo dijera, ¿verdad?"),
            ("___ furioso"," si te hicieran lo mismo a ti."),
        ],
        ("ponerse","condicional","él/ella") => &[
            ("Mi jefe ___"," hecho una furia si se enterara."),
            ("Ella ___"," el uniforme sin rechistar si se lo pidieran."),
            ("El bebé ___"," a llorar si le quitaras el juguete."),
        ],
        // ── llevarse ─────────────────────────────────────────────────────────
        ("llevarse","presente","yo") => &[
            ("Cuando salgo de casa, siempre ___"," el paraguas aunque no llueva."),
            ("___ bien con casi todo el mundo,"," salvo con mi cuñado."),
            ("De la librería, ___"," tres novelas cada vez — no tengo remedio."),
        ],
        ("llevarse","presente","tú") => &[
            ("¿___ bien con tus compañeros de piso?",""),
            ("Siempre ___ el cargador"," y luego me lo pides prestado."),
            ("___ la peor parte"," cada vez que hay que repartir el trabajo."),
        ],
        ("llevarse","presente","él/ella") => &[
            ("Mi gato ___"," todo lo que encuentra por el suelo."),
            ("Ella ___"," una decepción enorme con ese libro."),
            ("Él no ___"," bien con su jefe desde el principio."),
        ],
        ("llevarse","pretérito","yo") => &[
            ("___ el paraguas"," y menos mal — llovió todo el día."),
            ("Sin querer, ___"," el abrigo de mi hermano del perchero."),
            ("___ una sorpresa tremenda"," cuando vi quién estaba en la fiesta."),
        ],
        ("llevarse","pretérito","tú") => &[
            ("¿___ el paraguas ayer?"," Dicen que llovió mucho al norte."),
            ("___ un susto enorme"," cuando el perro salió corriendo."),
            ("___ las llaves por error"," y me quedé fuera toda la tarde."),
        ],
        ("llevarse","pretérito","él/ella") => &[
            ("El ladrón ___"," el bolso pero dejó el móvil."),
            ("Ella ___"," una alegría enorme al saber que había aprobado."),
            ("Mi vecino ___"," la carta que era para mí — otra vez."),
        ],
        ("llevarse","perfecto","yo") => &[
            ("___ el paraguas"," y aun así llegué empapado — no sirve de nada."),
            ("___ una decepción enorme"," con esa película que tanto recomendabas."),
            ("___ bien con la nueva jefa"," desde el primer día."),
        ],
        ("llevarse","perfecto","tú") => &[
            ("¿___ el cargador?"," El mío no funciona bien."),
            ("___ una sorpresa,"," ¿verdad? No esperabas verme aquí."),
            ("¿___ bien con ella"," durante el viaje?"),
        ],
        ("llevarse","perfecto","él/ella") => &[
            ("Alguien ___"," mi paraguas del perchero de la oficina."),
            ("Ella ___"," el primer premio en el concurso de fotografía."),
            ("Mi hermano nunca ___"," bien con los vecinos de arriba."),
        ],
        ("llevarse","condicional","yo") => &[
            ("___ el abrigo grueso,"," pero hace demasiado calor para eso."),
            ("___ bien con cualquiera"," si la gente fuera más honesta."),
            ("___ una alegría enorme"," si me llamaran para el puesto."),
        ],
        ("llevarse","condicional","tú") => &[
            ("¿___ el paraguas si vieras nubes?"," Yo sí."),
            ("___ mejor con él"," si no fuera tan cerrado de mente."),
            ("___ un disgusto enorme"," si perdieras esa cartera."),
        ],
        ("llevarse","condicional","él/ella") => &[
            ("Mi madre ___"," el bolso más grande si cupiera en el maletero."),
            ("Él ___"," una sorpresa si viera lo que han hecho con su despacho."),
            ("Cualquiera ___"," un susto en esa situación."),
        ],
        // ── acordarse ────────────────────────────────────────────────────────
        ("acordarse","presente","yo") => &[
            ("Nunca ___"," de dónde pongo las llaves."),
            ("___ de ti"," cada vez que escucho esa canción."),
            ("___ perfectamente"," del día que nos conocimos."),
        ],
        ("acordarse","presente","tú") => &[
            ("¿___ de traer el libro que te presté?",""),
            ("No ___"," de nada cuando estás nervioso, ¿verdad?"),
            ("¿___ de cuando íbamos juntos al cole?"," Qué tiempos."),
        ],
        ("acordarse","presente","él/ella") => &[
            ("Mi abuela ___"," de todos los cumpleaños sin agenda."),
            ("Él nunca ___"," de cerrar la puerta con llave."),
            ("Ella siempre ___"," de preguntar por tu familia — es un detalle."),
        ],
        ("acordarse","pretérito","yo") => &[
            ("Por suerte, ___"," de coger el paraguas antes de salir."),
            ("No ___ de su cumpleaños"," hasta que lo vi en el trabajo — qué vergüenza."),
            ("___ de repente"," de que había dejado el horno encendido."),
        ],
        ("acordarse","pretérito","tú") => &[
            ("¿___ de llamar al médico"," para pedir la cita?"),
            ("Justo cuando llegaste, ___"," de donde había puesto el contrato."),
            ("No ___ de apagar la luz,"," ¿a que no? Como siempre."),
        ],
        ("acordarse","pretérito","él/ella") => &[
            ("Mi madre ___"," de llamarme justo cuando iba a acostarme."),
            ("Él ___"," de ir al parque con el perro a pesar de la lluvia."),
            ("Ella por fin ___"," de devolverme el dinero que le presté."),
        ],
        ("acordarse","perfecto","yo") => &[
            ("___ de comprar el pan"," — está en la bolsa de la entrada."),
            ("Hoy por fin ___"," de hacer la transferencia que llevaba semanas postergando."),
            ("No ___ de llamarte antes,"," lo siento — ha sido un día loco."),
        ],
        ("acordarse","perfecto","tú") => &[
            ("¿___ de echar la llave?"," Creo que he oído la puerta."),
            ("___ de todo menos de lo importante,"," como siempre."),
            ("Por fin ___"," de devolverme el paraguas. Solo tardaste dos meses."),
        ],
        ("acordarse","perfecto","él/ella") => &[
            ("Mi hermano ___"," de ir al parque después de clase."),
            ("Ella ___"," de reservar mesa para el sábado — menos mal."),
            ("Nadie ___"," de comprar la tarta de cumpleaños."),
        ],
        ("acordarse","condicional","yo") => &[
            ("___ de todo"," si durmiera más de cinco horas."),
            ("No ___"," de ese día ni aunque quisiera — fue horrible."),
            ("___ de llamarte antes"," si tuviera tu número nuevo."),
        ],
        ("acordarse","condicional","tú") => &[
            ("¿___ de cerrar con llave"," si salieras el último?"),
            ("___ de él más a menudo"," si lo hubieras conocido mejor."),
            ("___ de traer el paraguas"," si hubieras visto el pronóstico."),
        ],
        ("acordarse","condicional","él/ella") => &[
            ("Mi abuelo ___"," de cada detalle si le preguntaras con calma."),
            ("Ella ___"," de tu nombre aunque solo os hubierais visto una vez."),
            ("Él nunca ___"," de nada aunque le dejaras cien notas."),
        ],
        _ => panic!("Missing sentences for {verb}/{tense}/{person}"),
    }
}