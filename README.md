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
---------------

Come sviluppare gli errori:
- considerare una lista di possibili componenti da 'attaccare' (valori di soglie, pesi delle sinapsi, potenziali di membrane)
- per attacco si intende la modifica secondo uno dei 3 errori di uno dei bit di tale valore (-> trasformare il valore in bit ed in seguito eseguire una bit-wise ed impostare il valore ottenuto. es. 5-> 101 bitwise stuck-at-0 sull'ultimo bit ottengo 100 =4)
- possibilità di errori: stuck-at-0, stuck-at-1, transient bit-flip
-indicare il numero di occorrenze di tali guasti ( cioè ripetere i guasti N volte su diversi bit dei diversi componenti da verificare)
- indicare la normale sequenza di input della rete stessa
