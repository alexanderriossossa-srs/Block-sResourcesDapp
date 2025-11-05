// Este archivo define la página principal de la aplicación, integrando los componentes de encabezado y tabla de valores.
import Header from "./components/Header";
import TablaValores from "./components/TablaValores";
import React from "react";

// Componente principal de la página Home
export default function Home() {
  return (
    // Estructura principal de la página con estilos y disposición
    <main className="min-h-screen bg-black flex flex-col items-center justify-start">
      {/* Componente de encabezado que gestiona la conexión de la billetera */}
      <Header />
      <section className="w-full max-w-2xl mt-10 px-4">
        <div className="bg-gray-900 bg-opacity-80 rounded-xl shadow-[0_4px_24px_0_rgba(255,255,255,0.15)] border border-gray-700 p-8">
          <h2 className="text-3xl font-bold text-white mb-2 text-center">Get Set Intro Bootcamp 2026</h2>
          <p className="text-gray-300 text-center mb-8">
            Esta aplicación permite recuperar y establecer un valor tipo string en un contrato inteligente sobre la red Stellar. 
          </p>
          <p className="text-gray-400 text-center mb-8 font-mono">
            Contrato: CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL
          </p>
          {/* Componente que muestra y permite actualizar el valor del contrato */}
          <TablaValores />
        </div>
      </section>
    </main>
  );
}