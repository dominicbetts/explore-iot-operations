/* Code generated by Azure.Iot.Operations.ProtocolCompiler; DO NOT EDIT. */

#nullable enable

namespace PTZ.dtmi_onvif_ptz__1
{
    using System;
    using System.Collections.Generic;
    using System.Text.Json.Serialization;
    using PTZ;

    public class RelativeMoveRequestPayload : IJsonOnDeserialized, IJsonOnSerializing
    {
        /// <summary>
        /// The Command request argument.
        /// </summary>
        [JsonPropertyName("RelativeMove")]
        [JsonIgnore(Condition = JsonIgnoreCondition.Never)]
        [JsonRequired]
        public Object_Onvif_Ptz_RelativeMove__1 RelativeMove { get; set; } = default!;

        void IJsonOnDeserialized.OnDeserialized()
        {
            if (RelativeMove is null)
            {
                throw new ArgumentNullException("RelativeMove field cannot be null");
            }
        }

        void IJsonOnSerializing.OnSerializing()
        {
            if (RelativeMove is null)
            {
                throw new ArgumentNullException("RelativeMove field cannot be null");
            }
        }
    }
}
