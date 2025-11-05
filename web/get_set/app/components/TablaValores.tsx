// Este componente permite leer y escribir un mensaje en un contrato inteligente de Stellar.
'use client';
import { nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import React, { useEffect, useState } from "react";
import toast from "react-hot-toast";
import { useRegisteredContract, useSorobanReact } from "stellar-react";

// Componente principal de la tabla de valores
const TablaValores: React.FC = () => {
  // Obtenemos información de la red, el servidor y la dirección de la billetera
  const { activeNetwork, sorobanServer, address } = useSorobanReact();
  // Obtenemos el contrato registrado con el nombre "get_set_message"
  const contract = useRegisteredContract("get_set_message");
  
  // Estado para el mensaje actual y el nuevo mensaje a enviar
  const [message, setMessage] = useState("");
  const [newMessage, setNewMessage] = useState("");

  // Función para leer el mensaje almacenado en el contrato inteligente
  const readMessage = async () => {
 
    if (!contract) {
      return;
    }
    try {
      // Obtenemos la dirección del contrato y llamamos al método get_message
      const addr = contract?.deploymentInfo?.contractAddress;
      const result = await contract?.invoke({ method: "get_message", args: [] });
      setMessage(scValToNative(result as xdr.ScVal) as string)
    }
    catch (error) {
      console.log(error);
    }
  }
  // Función para escribir un nuevo mensaje en el contrato inteligente
  const writeMessage = async () => {
    // Validamos que la billetera esté conectada
    if (address === undefined) {
      toast.error("Wallet no conectada");
      return;
    }
    // Validamos que el campo no esté vacío
    if (newMessage === "") {
      toast.error("Introduce un mensaje");
      return;
    }
    try {
      // Llamamos al método set_message del contrato, firmando la transacción
      const result = await contract?.invoke({
        method: "set_message",
        args: [nativeToScVal(newMessage, { type: "string" })],
        signAndSend: true,
        reconnectAfterTx: false
      });
      if (result) {
        toast.success("mensaje actualizado");
        readMessage(); // Actualizamos el mensaje mostrado
      } else {
        toast.error("Actuliazacion fallida");
      }
    } catch (error) {
      if (error instanceof Error) {
        console.log(`Error: ${error.message}`);
      }
      else {
        console.log('ocurrio un error desconocido');
      }
    }
  }
  
  // Hook useEffect para leer el mensaje cada vez que cambia el servidor o el contrato
  useEffect(() => {
    readMessage();
  }, [sorobanServer, contract]);
  return (
    // Tabla visual para mostrar y actualizar el valor del contrato
    <div className="w-full max-w-md mx-auto mt-8 bg-gray-900 bg-opacity-80 rounded-lg shadow-lg p-6">
      <table className="w-full text-white border-separate border-spacing-y-4">
        <tbody>
          <tr>
            <td className="font-semibold">Valor actual</td>
            <td className="bg-gray-800 rounded px-4 py-2 text-center">{message}</td>
          </tr>
          <tr>
            <td className="font-semibold">Valor nuevo</td>
            <td>
              <input
                type="text"
                className="w-full px-3 py-2 rounded bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-600"
                value={newMessage}
                onChange={e => setNewMessage(e.target.value)}
                placeholder="Introduce un nuevo valor"
              />
            </td>
          </tr>
          <tr>
            <td></td>
            <td>
              <button
                className="w-full bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded transition mt-2"
                onClick={writeMessage}
              >
                Enviar
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  );
};

// Exportamos el componente para su uso en otras partes de la app
export default TablaValores;