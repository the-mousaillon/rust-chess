import * as React from 'react';

export default function MoveMarker(props: {}) {
  return (
    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlnsXlink="http://www.w3.org/1999/xlink" preserveAspectRatio="xMidYMid meet" viewBox="0 0 640 640" width={35} height={35} {...props}>
      <defs>
        <path d="M472.46 302.13C472.46 394.28 399.55 469.1 309.75 469.1C219.95 469.1 147.05 394.28 147.05 302.13C147.05 209.98 219.95 135.16 309.75 135.16C399.55 135.16 472.46 209.98 472.46 302.13Z" id="dVZYn4fvb" />
      </defs>
      <g>
        <g>
          <g>
            <g>
              <filter id="shadow8093871" x="135.05" y="123.16" width="353.41" height="360.94" filterUnits="userSpaceOnUse" primitiveUnits="userSpaceOnUse">
                <feFlood />
                <feComposite in2="SourceAlpha" operator="in" />
                <feGaussianBlur stdDeviation={1} />
                <feOffset dx={4} dy={3} result="afterOffset" />
                <feFlood floodColor="#000000" floodOpacity="0.5" />
                <feComposite in2="afterOffset" operator="in" />
                <feMorphology operator="dilate" radius={1} />
                <feComposite in2="SourceAlpha" operator="out" />
              </filter>
              <path d="M472.46 302.13C472.46 394.28 399.55 469.1 309.75 469.1C219.95 469.1 147.05 394.28 147.05 302.13C147.05 209.98 219.95 135.16 309.75 135.16C399.55 135.16 472.46 209.98 472.46 302.13Z" id="ah3iXruC" fill="white" fillOpacity={1} filter="url(#shadow8093871)" />
            </g>
            <use xlinkHref="#dVZYn4fvb" opacity={1} fill="#c69e1a" fillOpacity={1} />
            <g>
              <use xlinkHref="#dVZYn4fvb" opacity={1} fillOpacity={0} stroke="#000000" strokeWidth={1} strokeOpacity={1} />
            </g>
          </g>
        </g>
      </g>
    </svg>
  );
}
