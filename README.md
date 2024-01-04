# Group25
cose da fare:
-sviluppare la NN
-fare parallelizzazione
-sviluppare il LIF
-sviluppare i comandi per i test sugli errori
-raccogliere i dati sugli errori per studiare la resilienza
-fare presentazione power point

All'interno della NN:
-sviluppare interfaccia neurone di tipo generico ed independete dal modello interno scelto; il neurone riceve impulsi di tipo binario e da in output impulsi di tipo binario
- i parametri interni (potenziale di reset, potenziale di riposo e soglia) devono essere configurabili
- utilizzare struttura fully connected
- numero di strati e neuroni totalmente configurabile

lif_neuron.rs:
- implementiamo il tratto Neuron in cui all'interno è presente un tipo ClassNeuron
- implementiamo una struct di tipo LIFNeuron
- implementiamo una struct LeakyIntegrateFire
- implementiamo il tratto Neuron per la struct LeakyIntegrateFire in cui il tipo ClassNeuron è LIFNeuron


---- bash ----
git add nomemodulo.rs
git commit -m "???"
git push

